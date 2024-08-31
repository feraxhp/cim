use image::{DynamicImage, ImageFormat};

pub(crate) fn image_to_png(img: DynamicImage, output_path: &str, _: f32) -> Result<(), Box<dyn std::error::Error>> {
    // save png
    img.save_with_format(output_path, ImageFormat::Png)?;

    Ok(())
}
