use std::error;
use std::path::PathBuf;
use image::{DynamicImage, RgbaImage};
use webp::Decoder;

pub(crate) fn load_webp(input_path: &PathBuf) -> Result<DynamicImage, Box<dyn error::Error>> {
    // Load WebP image
    let webp_data = std::fs::read(input_path)?;
    let decoder = Decoder::new(&webp_data);
    let webp_image = decoder.decode().unwrap();

    // Convert WebP image to DynamicImage
    let image = DynamicImage::ImageRgba8(RgbaImage::from(webp_image.to_image()));

    Ok(image)
}