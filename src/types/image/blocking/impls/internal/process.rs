use crate::{blocking::Image, image::blocking::ProcessedImage, Result};

impl Image {
    pub(crate) fn process_image(&mut self) -> Result<ProcessedImage<'_>> {
        self.ensure_decoded()?;

        let pipeline = std::mem::take(&mut self.config.pipeline);

        if pipeline.is_empty() {
            let img_ref = self.get_decoded();
            return Ok(ProcessedImage::Borrowed(img_ref));
        }

        let mut img = self.get_decoded().clone();

        for op in pipeline {
            op.apply(&mut img);
        }

        Ok(ProcessedImage::Owned(img))
    }

    pub(crate) fn process_image_in_place(&mut self) -> Result<()> {
        self.ensure_decoded()?;

        let pipeline = std::mem::take(&mut self.config.pipeline);
        let mut img = self.get_decoded_mut();

        for op in pipeline {
            op.apply(&mut img);
        }

        Ok(())
    }
}
