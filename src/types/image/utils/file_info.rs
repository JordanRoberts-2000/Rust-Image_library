use {
    crate::constants::DEFAULT_IMAGE_FILE_NAME,
    std::path::{Path, PathBuf},
};

pub fn file_info(path: &Path) -> (String, PathBuf) {
    let file_name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(DEFAULT_IMAGE_FILE_NAME)
        .to_string();

    let parent_dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), |p| p.to_path_buf());

    (file_name, parent_dir)
}

#[cfg(test)]
mod tests {
    use {
        super::{file_info, DEFAULT_IMAGE_FILE_NAME},
        std::path::{Path, PathBuf},
    };

    #[test]
    fn test_file_info_with_normal_path() {
        let path = Path::new("/some/path/picture.png");
        let (name, parent) = file_info(path);
        assert_eq!(name, "picture");
        assert_eq!(parent, PathBuf::from("/some/path"));
    }

    #[test]
    fn test_file_info_with_relative_path() {
        let path = Path::new("picture.jpg");
        let (name, parent) = file_info(path);
        assert_eq!(name, "picture");
        assert_eq!(parent, PathBuf::from("."));
    }

    #[test]
    fn test_file_info_with_non_utf8_name() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let bytes = b"\xFF\xFE"; // invalid UTF-8
        let path = Path::new(OsStr::from_bytes(bytes));
        let (name, parent) = file_info(path);
        assert_eq!(name, DEFAULT_IMAGE_FILE_NAME);
        assert_eq!(parent, PathBuf::from("."));
    }

    #[test]
    fn test_file_info_with_nested_path() {
        let path = Path::new("folder/subfolder/photo.jpeg");
        let (name, parent) = file_info(path);
        assert_eq!(name, "photo");
        assert_eq!(parent, PathBuf::from("folder/subfolder"));
    }
}
