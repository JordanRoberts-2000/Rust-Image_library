use {
    image::{DynamicImage, ImageReader},
    std::path::Path,
};

pub fn open_image(path: impl AsRef<Path>) -> DynamicImage {
    let path = path.as_ref();

    ImageReader::open(&path)
        .unwrap_or_else(|e| panic!("open input {:?}: {}", path.to_owned(), e))
        .with_guessed_format()
        .unwrap_or_else(|e| panic!("guess format for {:?}: {}", path.to_owned(), e))
        .decode()
        .unwrap_or_else(|e| panic!("decode {:?}: {}", path.to_owned(), e))
}
