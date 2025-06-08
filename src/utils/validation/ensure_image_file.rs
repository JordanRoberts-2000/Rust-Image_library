use std::path::Path;

use crate::{enums::ValidImageFormat, ValidationError};

pub fn ensure_existing_image_file(path: impl AsRef<Path>) -> Result<(), ValidationError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()));
    }
    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| ValidationError::MissingExtension(path.to_path_buf()))?;

    ValidImageFormat::try_from(ext)
        .map(|_| path.to_path_buf())
        .map_err(|_| ValidationError::NotAnImageFile(path.to_path_buf()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, path::PathBuf};
    use tempfile::TempDir;

    #[test]
    fn nonexistent_path_returns_path_not_found() {
        let fake = PathBuf::from("this/path/should/not/exist.img");
        let err = ensure_existing_image_file(&fake).unwrap_err();
        match err {
            ValidationError::PathNotFound(p) => assert_eq!(p, fake),
            _ => panic!("expected PathNotFound, got {:?}", err),
        }
    }

    #[test]
    fn directory_returns_not_a_file() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();
        let err = ensure_existing_image_file(dir).unwrap_err();
        match err {
            ValidationError::NotAFile(p) => assert_eq!(p, dir),
            _ => panic!("expected NotAFile, got {:?}", err),
        }
    }

    #[test]
    fn file_without_extension_returns_missing_extension() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("noext");
        File::create(&file_path).unwrap();
        let err = ensure_existing_image_file(&file_path).unwrap_err();
        match err {
            ValidationError::MissingExtension(p) => assert_eq!(p, file_path),
            _ => panic!("expected MissingExtension, got {:?}", err),
        }
    }

    #[test]
    fn unsupported_extension_returns_not_an_image_file() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("foo.txt");
        File::create(&file_path).unwrap();
        let err = ensure_existing_image_file(&file_path).unwrap_err();
        match err {
            ValidationError::NotAnImageFile(p) => assert_eq!(p, file_path),
            _ => panic!("expected NotAnImageFile, got {:?}", err),
        }
    }

    #[test]
    fn supported_extensions_are_accepted() {
        let tmp = TempDir::new().unwrap();
        for ext in &["png", "jpg", "jpeg", "webp", "JPG", "Png"] {
            let file_path = tmp.path().join(format!("img.{}", ext));
            File::create(&file_path).unwrap();
            ensure_existing_image_file(&file_path)
                .unwrap_or_else(|e| panic!("{} should be OK but got {:?}", ext, e));
        }
    }
}
