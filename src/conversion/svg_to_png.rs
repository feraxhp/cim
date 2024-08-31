use resvg::usvg::{Tree, Options, tiny_skia_path};
use std::fs;

pub(crate) fn svg_to_png(
    svg_path: &str,
    export_path: &str,
    width: u32,
    height: u32
) -> Result<(), Box<dyn std::error::Error>> {

    // load SVG
    let svg_data = fs::read(svg_path)?;

    let options = Options::default();
    let tree = Tree::from_data(&svg_data, &options)?;

    let width = if width == 0 { tree.size().width() as u32 } else { width };
    let height = if height == 0 { tree.size().height() as u32 } else { height };

    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("Incorrect svg format")?;
    let mut pixmapmut = pixmap.as_mut();

    let sy: f32 = (width as f32)/tree.size().width();
    let sx: f32 = (height as f32)/tree.size().height();

    resvg::render(&tree, tiny_skia_path::Transform::from_scale(sx, sy), &mut pixmapmut);

    // save PNG
    pixmap.save_png(export_path)?;

    Ok(())
}
