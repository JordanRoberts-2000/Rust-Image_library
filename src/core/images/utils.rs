use {
    crate::{Image, Result},
    std::path::Path,
};

pub fn try_load_image(path: &Path, skip_errors: bool) -> Result<Option<Image>> {
    match Image::from_file(path) {
        Ok(img) => Ok(Some(img)),
        Err(_) if skip_errors => Ok(None),
        Err(e) => Err(e),
    }
}
