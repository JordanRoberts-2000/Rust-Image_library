use {razr::ImageFormat, std::path::PathBuf};

pub fn get_format(format: ImageFormat) -> PathBuf {
    let assets_dir = PathBuf::from("tests/assets/formats");

    match format {
        ImageFormat::Avif => assets_dir.join("image.avif"),
        ImageFormat::Jpeg => assets_dir.join("image.jpeg"),
        ImageFormat::Png => assets_dir.join("image.png"),
        ImageFormat::WebP => assets_dir.join("image.webp"),
    }
}
