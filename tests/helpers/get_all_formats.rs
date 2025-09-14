use std::{fs, path::PathBuf};

pub fn get_all_formats() -> Vec<PathBuf> {
    let assets_dir = "tests/assets/formats";

    let entries = fs::read_dir(assets_dir).expect("Failed to read assets directory");

    let mut image_paths = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        image_paths.push(path);
    }

    image_paths
}
