use {crate::FormatFilter, fs_ext::DirQueryOptions};

pub struct FromFolderConfig {
    pub(crate) recursive: bool,
    pub(crate) max_depth: Option<usize>,
    pub(crate) limit: Option<usize>,
    pub(crate) format_filter: Option<FormatFilter>,
    pub(crate) skip_errors: bool,
}

impl Default for FromFolderConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            max_depth: None,
            limit: None,
            format_filter: None,
            skip_errors: false,
        }
    }
}

impl From<FromFolderConfig> for DirQueryOptions {
    fn from(cfg: FromFolderConfig) -> DirQueryOptions {
        DirQueryOptions {
            include_files: true,
            include_dirs: false,
            recursive: cfg.recursive,
            limit: cfg.limit,
            depth: cfg.max_depth,
            extension_filter: cfg.format_filter.map(Into::into),
        }
    }
}

impl From<&FromFolderConfig> for DirQueryOptions {
    fn from(cfg: &FromFolderConfig) -> DirQueryOptions {
        DirQueryOptions {
            include_files: true,
            include_dirs: false,
            recursive: cfg.recursive,
            limit: cfg.limit,
            depth: cfg.max_depth,
            extension_filter: cfg.format_filter.clone().map(Into::into),
        }
    }
}
