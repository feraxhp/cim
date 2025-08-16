use std::path::PathBuf;
use image::DynamicImage;
use super::load_image::load_image;
use super::load_svg::load_svg;
use super::load_webp::load_webp;

pub(crate) async fn load_any_image(
    input: &PathBuf,
    width: u32,
    height: u32
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let image = match input.extension().unwrap().to_str().unwrap()
    {
        "svg" => load_svg(&input, width, height).await?,
        "webp" => load_webp(&input).await?,
        _ => load_image(&input).await?,
    };
    Ok(image)
}
