use {
    crate::{image::Decoded, ErrorKind, Result},
    image::{DynamicImage, RgbaImage},
    resvg::tiny_skia::Pixmap,
    std::io::{BufRead, Seek},
    usvg::{Transform, Tree},
};

pub fn rasterize_svg<R: BufRead + Seek>(reader: &mut R) -> Result<Decoded> {
    let mut svg_bytes = Vec::new();
    reader.read_to_end(&mut svg_bytes)?;

    let opt = usvg::Options::default();
    let tree = Tree::from_data(&svg_bytes, &opt).map_err(|_| ErrorKind::SvgRaster)?;

    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;

    let mut pixmap = Pixmap::new(width, height).ok_or(ErrorKind::SvgRaster)?;

    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

    let img = RgbaImage::from_raw(width, height, pixmap.take()).ok_or(ErrorKind::SvgRaster)?;

    Ok(Decoded::Static(DynamicImage::ImageRgba8(img)))
}
