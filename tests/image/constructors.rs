use img::{blocking::Image as SyncImage, ImageError};

// #[test]
// fn test_image_from_file_success() -> Result<(), ImageError> {
//     let image = SyncImage::from_file("./playground/image.png")?;

//     assert_eq!(image.width > 0, true);
//     assert_eq!(image.height > 0, true);
//     assert_eq!(image.aspect_ratio > 0.0, true);

//     Ok(())
// }

// #[tokio::test]
// async fn test_async_image_from_file_success() -> Result<(), ImageError> {
//     let image = Image::from_file("./playground/image.png").await?;

//     assert!(image.width > 0);
//     assert!(image.height > 0);
//     assert!(image.aspect_ratio > 0.0);

//     Ok(())
// }
