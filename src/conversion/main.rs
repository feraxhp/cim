use std::path::PathBuf;
use tokio::fs;

use color_print::cformat;
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


pub async fn convert(input: &PathBuf, output: &PathBuf, options: &Options) -> Result<(String, String), String> {
    let image = match load_any_image(input, options.width, options.height).await {
        Ok(image) => image,
        Err(error) => {
            let name = input.to_str().unwrap();
            return Err(
                vec![
                    cformat!("<r>⨯ Error:</> loading: <b,u>{}</>", name),
                    cformat!("  → {:#?}", error)
                ].join("\n")
            )
        }
    };
    
    let output = match output.extension() {
        None => {
            let input_ = input.file_stem().unwrap().to_str().unwrap();
            output.join(format!("{}.{}", input_, &options.format))
        }
        Some(_) => {
            output.clone()
        }
    };
    
    let parent = output.parent().unwrap();
    fs::create_dir_all(parent).await.unwrap();
    
    let input_str = input.as_os_str().to_str().unwrap().to_string();
    let output_str = output.as_path().to_str().unwrap().to_string();

    match options.format.as_str() {
        "webp" => {
            match save_as_webp(image, &output, options.quality).await {
                Ok(_) => Ok((input_str, output_str)),
                Err(error) => Err(format!("{:#?}", error))
            }
        }
        _ => {
            let format = ImageFormat::from_extension(&options.format).unwrap();
            match save_as_format(image, &output, format).await {
                Ok(_) => Ok((input_str, output_str)),
                Err(error) => Err(format!("{:#?}", error))
            }
        }
    }
}