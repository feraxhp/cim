use std::path::PathBuf;
use image::DynamicImage;

pub(crate) fn load_image(input_path: &PathBuf) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    match image::ImageReader::open(input_path) {
        Ok(reader) => match reader.decode() {
            Ok(image) => Ok(image),
            Err(e) => Err(Box::new(e))
        },
        Err(e) => Err(Box::new(e))
    }
}
