use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::{Image, IoError, Result};

impl Image {
    pub async fn write_encoded<W>(writer: W, data: Vec<u8>, id: String) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut buf_writer = BufWriter::new(writer);

        buf_writer
            .write_all(&data)
            .await
            .map_err(|e| IoError::WriteAll(e, id.clone()))?;

        buf_writer
            .flush()
            .await
            .map_err(|e| IoError::Flush(e, id.clone()))?;

        Ok(())
    }
}
