use {
    crate::helpers::{get_all_formats, open_image},
    image::GenericImageView,
    razr::JpegEncoder,
    std::fs::{self, File},
    tempfile::tempdir,
};

#[test]
fn test_jpeg_encoder() {
    let paths = get_all_formats();
    let out_dir = tempdir().expect("create temporary output directory");
    let encoder = JpegEncoder::new();

    for (i, path) in paths.into_iter().enumerate() {
        let out_path = out_dir.path().join(format!("img_{}.jpg", i));
        let file = File::create(&out_path)
            .unwrap_or_else(|e| panic!("create output {:?}: {}", out_path, e));

        let img = open_image(&path);
        let (height, width) = img.dimensions();

        encoder.encode(file, img.as_bytes(), width, height).expect("msg");

        let meta =
            fs::metadata(&out_path).unwrap_or_else(|e| panic!("stat output {:?}: {}", out_path, e));
        assert!(meta.len() > 0, "encoded file {:?} is empty", out_path);
    }
}

#[test]
fn test_jpeg_progressive_encoder() {
    let paths = get_all_formats();
    let out_dir = tempdir().expect("create temporary output directory");
    let encoder = JpegEncoder::progressive();

    for (i, path) in paths.into_iter().enumerate() {
        let out_path = out_dir.path().join(format!("img_{}.jpg", i));
        let file = File::create(&out_path)
            .unwrap_or_else(|e| panic!("create output {:?}: {}", out_path, e));

        let img = open_image(&path);
        let (height, width) = img.dimensions();

        encoder.encode(file, img.as_bytes(), width, height).expect("msg");

        let meta =
            fs::metadata(&out_path).unwrap_or_else(|e| panic!("stat output {:?}: {}", out_path, e));
        assert!(meta.len() > 0, "encoded file {:?} is empty", out_path);
    }
}
