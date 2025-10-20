use {
    super::{decode_gif, rasterize_svg},
    crate::{image::Decoded, ErrorKind, Format, Result},
    image::ImageFormat,
    std::io::{BufRead, Seek},
};

pub fn from_reader<R: BufRead + Seek>(reader: &mut R, format: &Format) -> Result<Decoded> {
    use image::ImageReader as IR;
    let decoded = match *format {
        Format::Svg => rasterize_svg(reader)?,
        Format::Gif => decode_gif(reader)?,
        Format::Webp => Decoded::Static(
            IR::with_format(reader, ImageFormat::WebP).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Jpeg => Decoded::Static(
            IR::with_format(reader, ImageFormat::Jpeg).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Png => Decoded::Static(
            IR::with_format(reader, ImageFormat::Png).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Tiff => Decoded::Static(
            IR::with_format(reader, ImageFormat::Tiff).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Avif => Decoded::Static(
            IR::with_format(reader, ImageFormat::Avif).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Pnm => Decoded::Static(
            IR::with_format(reader, ImageFormat::Pnm).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Tga => Decoded::Static(
            IR::with_format(reader, ImageFormat::Tga).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Dds => Decoded::Static(
            IR::with_format(reader, ImageFormat::Dds).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Bmp => Decoded::Static(
            IR::with_format(reader, ImageFormat::Bmp).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Ico => Decoded::Static(
            IR::with_format(reader, ImageFormat::Ico).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Hdr => Decoded::Static(
            IR::with_format(reader, ImageFormat::Hdr).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::OpenExr => Decoded::Static(
            IR::with_format(reader, ImageFormat::OpenExr).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Farbfeld => Decoded::Static(
            IR::with_format(reader, ImageFormat::Farbfeld).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Qoi => Decoded::Static(
            IR::with_format(reader, ImageFormat::Qoi).decode().map_err(ErrorKind::Decode)?,
        ),
        Format::Pcx => Decoded::Static(
            IR::with_format(reader, ImageFormat::Pcx).decode().map_err(ErrorKind::Decode)?,
        ),
    };

    Ok(decoded)
}
