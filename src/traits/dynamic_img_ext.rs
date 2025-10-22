use {
    image::{Delay, DynamicImage, Frame},
    std::time::Duration,
};

pub trait DynamicImageExt {
    fn into_frame(self) -> Frame;
}

impl DynamicImageExt for DynamicImage {
    fn into_frame(self) -> Frame {
        let buffer = self.into_rgba8();
        Frame::from_parts(buffer, 0, 0, Delay::from_saturating_duration(Duration::from_millis(0)))
    }
}
