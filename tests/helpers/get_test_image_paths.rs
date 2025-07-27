use std::fs;

pub fn get_test_image_paths() -> Vec<std::path::PathBuf> {
    let assets_dir = "tests/assets";

    let entries = fs::read_dir(assets_dir).expect("Failed to read assets directory");

    let mut image_paths = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        image_paths.push(path);
    }

    image_paths
}
