use {
    crate::{image::Decoded, ErrorKind, Result},
    image::{codecs::gif::GifDecoder, AnimationDecoder, ImageDecoder},
    nonempty::NonEmpty,
    std::io::{BufRead, Seek},
};

pub fn decode_gif<R: BufRead + Seek>(reader: &mut R) -> Result<Decoded> {
    let decoder = GifDecoder::new(reader).map_err(ErrorKind::Decode)?;
    let (w, h) = decoder.dimensions();
    let frames_iter = decoder.into_frames();
    let frames = frames_iter.collect_frames().map_err(ErrorKind::Decode)?;

    let non_empty_frames = NonEmpty::from_vec(frames).ok_or(ErrorKind::EmptyGif)?;

    Ok(Decoded::Animated { frames: non_empty_frames, width: w, height: h })
}
