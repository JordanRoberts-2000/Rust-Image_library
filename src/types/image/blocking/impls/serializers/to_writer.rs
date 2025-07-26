use {
    crate::{blocking::Image, Result},
    std::io::Write,
};

impl Image {
    pub fn to_writer(&mut self, writer: impl Write) -> Result<()> {
        self.encode(writer, self.format)
    }
}
