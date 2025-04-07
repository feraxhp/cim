use std::path::PathBuf;
use image::{Rgba, Rgb, ImageBuffer, ImageFormat, DynamicImage};

fn rgba_to_rgb(rgba: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = rgba.dimensions();
    let mut rgb_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let alpha = pixel[3] as f32 / 255.0;
        let red = (pixel[0] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;
        let green = (pixel[1] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;
        let blue = (pixel[2] as f32 * alpha + 255.0 * (1.0 - alpha)) as u8;
        let rgb_pixel = Rgb([red, green, blue]);
        rgb_image.put_pixel(x, y, rgb_pixel);
    }

    rgb_image
}

pub(crate) fn save_as_format(
    img: DynamicImage,
    output: &PathBuf,
    format: ImageFormat
) -> Result<(), Box<dyn std::error::Error>> {
    let rgb_img = if format == ImageFormat::Jpeg {
        // color scheme conversion (rgba -> rgb)
        match img {
            DynamicImage::ImageRgba8(rgba) => DynamicImage::ImageRgb8(rgba_to_rgb(rgba)),
            _ => DynamicImage::from(img.to_rgb8()),
        }
    } else {
        img
    };

    // save image with specified Format
    rgb_img.save_with_format(output, format)?;

    Ok(())
}