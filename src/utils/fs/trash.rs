use std::path::Path;

use crate::{IoError, Result, ValidationError};

pub fn trash_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ValidationError::PathNotFound(path.to_path_buf()).into());
    }

    if !path.is_file() {
        return Err(ValidationError::NotAFile(path.to_path_buf()).into());
    }

    if let Err(trash_err) = trash::delete(path) {
        log::warn!(
            "Failed to trash '{}'. Falling back to permanent delete. err: {:?}",
            path.display(),
            trash_err
        );

        std::fs::remove_file(path).map_err(|e| IoError::DeleteFile(e, path.to_path_buf()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ImgError;

    use super::*;
    use std::{fs::File, path::PathBuf};
    use tempfile::TempDir;

    #[test]
    fn test_trash_file_nonexistent_path() {
        let path = PathBuf::from("does_not_exist.foo");
        let err = trash_file(&path).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::PathNotFound(p)) => {
                assert_eq!(p, path);
            }
            _ => panic!("expected PathNotFound, got {:?}", err),
        }
    }

    #[test]
    fn test_trash_file_on_directory() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();
        let err = trash_file(dir).unwrap_err();
        match err {
            ImgError::Validation(ValidationError::NotAFile(p)) => {
                assert_eq!(p, dir.to_path_buf());
            }
            _ => panic!("expected NotAFile, got {:?}", err),
        }
    }

    #[test]
    fn test_trash_file_removes_file() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("foo.txt");
        File::create(&file_path).unwrap();
        assert!(file_path.exists(), "precondition: file should exist");

        trash_file(&file_path).expect("should succeed");

        assert!(
            !file_path.exists(),
            "after trash_file, the file should no longer exist"
        );
    }
}
