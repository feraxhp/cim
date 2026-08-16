use resvg::usvg::{Tree, Options, tiny_skia_path};
use tokio::fs;
use std::path::PathBuf;
use image::{DynamicImage, RgbaImage};

use crate::conversion::utils::rezise::Size;


pub(crate) async fn load_svg(
    svg_path: &PathBuf,
    size: Option<Size>,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {

    // load SVG
    let svg_data = fs::read(svg_path).await?;

    let options = Options::default();
    let tree = Tree::from_data(&svg_data, &options)?;

    let size = size.unwrap_or_else(|| {
        let size = tree.size();
        Size::new(
            size.width() as u32,
            size.height() as u32,
        )
    });

    let width = size.width();
    let height = size.height();

    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("Incorrect svg Format")?;
    let mut pixmapmut = pixmap.as_mut();

    let sy: f32 = (width as f32)/tree.size().width();
    let sx: f32 = (height as f32)/tree.size().height();

    resvg::render(&tree, tiny_skia_path::Transform::from_scale(sx, sy), &mut pixmapmut);

    let image = DynamicImage::ImageRgba8(
        RgbaImage::from_raw(width, height, pixmap.data().to_vec())
            .ok_or("Failed to convert pixmap to DynamicImage")?
    );

    Ok(image)
}
