use image::{DynamicImage, EncodableLayout};
use webp::Encoder;

pub(crate) fn image_to_webp(img: DynamicImage, output_path: &str, quality: f32) -> Result<(), Box<dyn std::error::Error>> {
    let encoder = Encoder::from_image(&img)?;
    let webp_data = encoder.encode(quality); // Calidad de compresión

    // save WebP
    std::fs::write(output_path, webp_data.as_bytes())?;

    Ok(())
}