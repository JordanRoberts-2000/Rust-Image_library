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
        .map_or_else(|| PathBuf::from("."), |p| p.to_path_buf());

    (file_name, parent_dir)
}
