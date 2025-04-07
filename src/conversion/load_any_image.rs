use std::path::PathBuf;
use image::DynamicImage;
use crate::conversion::load_image::load_image;
use crate::conversion::load_svg::load_svg;
use crate::conversion::load_webp::load_webp;

pub(crate) fn load_any_image(
    input: &PathBuf,
    width: u32,
    height: u32
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let image = match input.extension().unwrap().to_str().unwrap()
    {
        "svg" => load_svg(&input, width, height)?,
        "webp" => load_webp(&input)?,
        _ => load_image(&input)?,
    };
    Ok(image)
}
