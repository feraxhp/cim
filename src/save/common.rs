use image::{DynamicImage, ImageFormat};
use crate::conversion::rgba2rgb;

pub(crate) fn save(
    img: DynamicImage,
    output_path: &str,
    format: ImageFormat
)  -> Result<(), Box<dyn std::error::Error>> {

    let dimage = match format {
        ImageFormat::Jpeg => {
            match img {
                DynamicImage::ImageRgba8(image) =>
                    DynamicImage::ImageRgb8(rgba2rgb::convert(image)),
                _ => DynamicImage::from(img.to_rgb8()),
            }
        }
        _ => img
    };

    dimage.save_with_format(output_path, format)?;

    Ok(())
}
