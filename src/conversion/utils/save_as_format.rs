use std::path::PathBuf;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Rgb, Rgba};
use std::io::Cursor; 
use tokio::fs;

use crate::conversion::utils::rezise::{resize, Size};

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

pub(crate) async fn save_as_format(
    img: DynamicImage,
    output: &PathBuf,
    format: ImageFormat
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut warning = None;
    let rgb_img = match format {
        ImageFormat::Jpeg => {
            // color scheme conversion (rgba -> rgb)
            match img {
                DynamicImage::ImageRgba8(rgba) => DynamicImage::ImageRgb8(rgba_to_rgb(rgba)),
                _ => DynamicImage::from(img.to_rgb8()),
            }
        },
        ImageFormat::Ico => {
            let size = img.dimensions();
            
            if size.0 > 256 { warning = Some(format!("resized to 256x256 due to ICO image restrictions")) };
            if size.1 > 256 { warning = Some(format!("resized to 256x256 due to ICO image restrictions")) };
            
            resize(img, Size::new(256, 256))
        },
        _ => img
    };
    
    let mut buffer = Cursor::new(Vec::new());
    rgb_img.write_to(&mut buffer, format)?;
    
    fs::write(output, buffer.get_ref()).await?;
    Ok(warning)
}