use {
    crate::{
        blocking::{Image, Images},
        images::types::FromFolderConfig,
        ImageError, ImageFormat,
    },
    std::path::Path,
    walkdir::{DirEntry, Error as WalkDirError, WalkDir},
};

impl Images {
    pub fn from_folder(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Self::from_folder_with_config(path, FromFolderConfig::default())
    }

    pub fn from_folder_with_config(
        path: impl AsRef<Path>, config: FromFolderConfig,
    ) -> Result<Self, ImageError> {
        let path = path.as_ref();
        let mut images = Vec::new();

        let walker = Self::create_walker(path, &config);

        for entry in walker {
            let entry = Self::resovle_entry(entry, &config).map_err(ImageError::WalkDir)?;

            if let Some(entry) = entry {
                let path = entry.path();

                if !Self::is_valid_image_file(path) {
                    continue;
                }

                if !Self::passes_filter(path, &config) {
                    continue;
                }

                if !Self::passes_format_checks(path, &config) {
                    continue;
                }

                match Self::try_load_image(path, &config)? {
                    Some(img) => images.push(img),
                    None => continue,
                }

                if images.len() >= config.limit {
                    break;
                }
            }
        }

        Ok(Self::from_vec(images))
    }

    fn create_walker<'a>(
        path: impl AsRef<Path> + 'a, config: &'a FromFolderConfig,
    ) -> impl Iterator<Item = Result<DirEntry, WalkDirError>> + 'a {
        WalkDir::new(path).max_depth(config.max_depth).follow_links(false).into_iter().filter_entry(
            move |entry| {
                if !config.recursive && entry.depth() > 1 {
                    return false;
                }
                true
            },
        )
    }

    fn resovle_entry(
        entry: Result<DirEntry, WalkDirError>, config: &FromFolderConfig,
    ) -> Result<Option<DirEntry>, WalkDirError> {
        match entry {
            Ok(e) => Ok(Some(e)),
            Err(e) => {
                if config.skip_errors {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    fn passes_filter(path: &Path, config: &FromFolderConfig) -> bool {
        if let Some(ref filter) = config.filter {
            (filter)(path)
        } else {
            true
        }
    }

    fn passes_format_checks(path: &Path, config: &FromFolderConfig) -> bool {
        let ext_format = Self::get_image_format_from_path(path);

        if let Some(format) = ext_format {
            Self::is_format_allowed(format, config)
        } else {
            // If filtering is enabled and format is unknown, skip
            config.valid_formats.is_none()
        }
    }

    fn is_valid_image_file(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        Self::get_image_format_from_path(path).is_some()
    }

    fn get_image_format_from_path(path: &Path) -> Option<ImageFormat> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| ImageFormat::try_from(ext).ok())
    }

    fn is_format_allowed(format: ImageFormat, config: &FromFolderConfig) -> bool {
        // Check if format is excluded
        if let Some(ref excluded) = config.exclude_formats {
            if excluded.contains(&format) {
                return false;
            }
        }

        // Check if format is in allowed list
        if let Some(ref allowed) = config.valid_formats {
            allowed.contains(&format)
        } else {
            true
        }
    }

    fn try_load_image(path: &Path, config: &FromFolderConfig) -> Result<Option<Image>, ImageError> {
        match Image::from_file(path) {
            Ok(img) => Ok(Some(img)),
            Err(e) => {
                if config.skip_errors {
                    Ok(None) // Skip this image, continue processing
                } else {
                    Err(e) // Propagate the error, stop processing
                }
            }
        }
    }
}
