use image::{ImageBuffer, Rgb, Rgba};

pub(crate) fn convert(rgba: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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