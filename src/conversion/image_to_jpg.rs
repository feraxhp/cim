use image::{Rgba, Rgb, ImageBuffer, ImageFormat, DynamicImage};

fn rgba_to_rgb(rgba: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = rgba.dimensions();
    let mut rgb_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let rgb_pixel = Rgb([pixel[0], pixel[1], pixel[2]]);
        rgb_image.put_pixel(x, y, rgb_pixel);
    }

    rgb_image
}

pub(crate) fn image_to_jpg(img: DynamicImage, output_path: &str, _: f32) -> Result<(), Box<dyn std::error::Error>> {
    // color scheme conversion (rgba -> rgb)
    let rgb_img = match img {
        DynamicImage::ImageRgba8(rgba) =>
            DynamicImage::ImageRgb8(rgba_to_rgb(rgba)), _ =>
                DynamicImage::from(img.to_rgb8()),
    };

    // save jpg
    rgb_img.save_with_format(output_path, ImageFormat::Jpeg)?;

    Ok(())
}