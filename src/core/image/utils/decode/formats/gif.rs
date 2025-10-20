use {
    crate::{image::Decoded, ErrorKind, Result},
    image::{codecs::gif::GifDecoder, AnimationDecoder, ImageDecoder},
    std::io::{BufRead, Seek},
};

pub fn decode_gif<R: BufRead + Seek>(reader: &mut R) -> Result<Decoded> {
    let decoder = GifDecoder::new(reader).map_err(ErrorKind::Decode)?;
    let (w, h) = decoder.dimensions();
    let frames_iter = decoder.into_frames();

    let frames = frames_iter.collect_frames().map_err(ErrorKind::Decode)?;
    if frames.is_empty() {
        return Err(ErrorKind::EmptyGif.into());
    }
    Ok(Decoded::Animated { frames, width: w, height: h })
}
