use crate::PngCompressionType;

pub struct PngEncoder {
    pub(super) compression_type: PngCompressionType,
}

impl PngEncoder {
    pub fn new() -> Self {
        Self { compression_type: PngCompressionType::Default }
    }

    pub fn best_compression() -> Self {
        Self { compression_type: PngCompressionType::Best }
    }

    pub fn fast() -> Self {
        Self { compression_type: PngCompressionType::Fast }
    }

    pub fn with_compression_type(mut self, compression_type: PngCompressionType) -> Self {
        self.compression_type = compression_type;
        self
    }
}

impl Default for PngEncoder {
    fn default() -> Self {
        Self::new()
    }
}
