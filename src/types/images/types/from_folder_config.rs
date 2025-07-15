use {crate::ImageFormat, std::path::Path};

pub struct FromFolderConfig {
    pub(crate) recursive: bool,
    pub(crate) max_depth: usize,
    pub(crate) filter: Option<Box<dyn Fn(&Path) -> bool + Send + 'static>>,
    pub(crate) valid_formats: Option<Vec<ImageFormat>>,
    pub(crate) exclude_formats: Option<Vec<ImageFormat>>,
    pub(crate) limit: usize,
    pub(crate) skip_errors: bool,
}

impl Default for FromFolderConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            max_depth: usize::MAX,
            filter: None,
            valid_formats: None,
            exclude_formats: None,
            limit: usize::MAX,
            skip_errors: false,
        }
    }
}
