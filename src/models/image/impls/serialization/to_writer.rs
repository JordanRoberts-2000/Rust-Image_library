use std::io::Write;

use crate::{Image, Result};

impl Image {
    pub fn to_writer(&mut self, writer: impl Write) -> Result<()> {
        self.apply_transforms()?;
        self.encode(writer)
    }
}
