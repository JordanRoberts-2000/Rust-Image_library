use std::io::{BufWriter, Write};

use crate::{blocking::Image, IoError, Result};

impl Image {
    pub(crate) fn write_encoded(writer: impl Write, data: &[u8], context: String) -> Result<()> {
        let mut buf_writer = BufWriter::new(writer);

        buf_writer
            .write_all(data)
            .map_err(|e| IoError::WriteAll(e, context.clone()))?;

        buf_writer.flush().map_err(|e| IoError::Flush(e, context))?;

        Ok(())
    }
}
