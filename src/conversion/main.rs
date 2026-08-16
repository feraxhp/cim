use std::path::PathBuf;
use tokio::fs;

use image::ImageFormat;


use crate::conversion::utils::rezise::{Size, resize};

use super::utils::load_any_image::load_any_image;
use super::utils::save_as_webp::save_as_webp;
use super::utils::save_as_format::save_as_format;


#[derive(Debug, Clone)]
pub struct Options {
    pub format: String,
    pub size: Option<Size>,
    pub quality: f32
}

#[derive(Debug, Clone)]
pub struct Output {
    pub path: String,
    pub warning: Option<String>
}

pub async fn convert(input: &PathBuf, output: &PathBuf, options: &Options) -> Result<Output, Box<dyn std::error::Error>> {
    
    let image = load_any_image(input, options.size.clone()).await?;
    
    let image = match options.size.clone() {
        Some(size) => resize(image, size),
        None => image,
    };
    
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
        "webp" => save_as_webp(image, &output, options.quality).await
            .map(|_| Ok(Output { path: output_str, warning: None }))?,
        _ => {
            let format = ImageFormat::from_extension(&options.format).unwrap();
            save_as_format(image, &output, format).await
                .map(|warning| Ok(Output { path: output_str, warning }))?
        }
    }
}