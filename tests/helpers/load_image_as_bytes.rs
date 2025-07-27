use std::{fs::File, io::Read, path::Path};

pub fn load_image_as_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    let path = path.as_ref();
    let mut file =
        File::open(path).expect(&format!("Failed to open file at path: {:?}", path.to_path_buf()));
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect(&format!("Failed to read file at path: {:?}", path.to_path_buf()));
    buffer
}
