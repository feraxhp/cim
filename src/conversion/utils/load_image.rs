use std::path::PathBuf;
use image::{DynamicImage, ImageFormat, ImageReader};
use std::io::Cursor;
use tokio::fs;

pub(crate) async fn load_image(input_path: &PathBuf) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let bytes = fs::read(input_path).await?;
    let format = ImageFormat::from_path(input_path)?;
    let image = ImageReader::with_format(Cursor::new(bytes), format).decode()?; // <--- 2. Decodifica desde memoria
    Ok(image)
}
