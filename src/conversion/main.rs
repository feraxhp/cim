use std::path::PathBuf;
use tokio::fs;

use image::ImageFormat;

use super::utils::load_any_image::load_any_image;
use super::utils::save_as_webp::save_as_webp;
use super::utils::save_as_format::save_as_format;


#[derive(Debug, Clone)]
pub struct Options {
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub quality: f32
}


pub async fn convert(input: &PathBuf, output: &PathBuf, options: &Options) -> Result<String, Box<dyn std::error::Error>> {
    let image = load_any_image(input, options.width, options.height).await?;
    
    let output = match output.extension() {
        None => {
            let input_ = input.file_stem().unwrap().to_str().unwrap();
            output.join(format!("{}.{}", input_, &options.format))
        }
        Some(_) => output.clone()
    };
    
    let parent = output.parent().unwrap();
    fs::create_dir_all(parent).await.unwrap();
    
    let output_str = output.as_path().to_str().unwrap().to_string();

    match options.format.as_str() {
        "webp" => save_as_webp(image, &output, options.quality).await.map(|_| Ok(output_str))?,
        _ => {
            let format = ImageFormat::from_extension(&options.format).unwrap();
            save_as_format(image, &output, format).await.map(|_| Ok(output_str))?
        }
    }
}