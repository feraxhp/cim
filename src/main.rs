mod conversion;
mod commands;
mod tools;

use clap::crate_version;
use color_print::cprintln;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs, io};
use std::io::Write;
use image::{DynamicImage, ImageFormat};
use crate::conversion::load_any_image::load_any_image;
use crate::conversion::save_as_format::save_as_format;
use crate::conversion::save_as_webp::save_as_webp;
use crate::tools::files::get_images;
use crate::tools::paths::path_to_absolute;


fn main() {
    let conversion = commands::base::command().get_matches();

    match conversion.clone().args_present() {
        true => {
            if *conversion.get_one::<bool>("vnumber").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }

    let format = conversion.get_one::<String>("format").unwrap().to_lowercase();
    let width = conversion.get_one::<u32>("width").unwrap();
    let height = conversion.get_one::<u32>("height").unwrap();
    let quality = conversion.get_one::<f32>("quality").unwrap();

    let input = match conversion.get_one::<PathBuf>("input") {
        Some(path_) => match path_to_absolute(&path_) {
            Ok(path_) => { path_ }
            Err(error) => {
                cprintln!("<r>* Error: Could not determine the input path</>");
                cprintln!("  → {}", error);
                exit(1);
            }
        },
        None => {
            cprintln!("<r>* Error: Could not determine the input path</>");
            exit(1);
        }
    };

    let output = match conversion.get_one::<PathBuf>("output") {
        Some(path_) => match  path_to_absolute(path_) {
            Ok(path_) => { path_ }
            Err(error) => {
                cprintln!("<r>* Error: Could not get the output path</>");
                cprintln!("  → {}", error);
                exit(1);
            }
        },
        None => match env::current_dir() {
            Ok(path) => { path }
            Err(error) => {
                cprintln!("<r>* Error: Could not get the output path</>");
                cprintln!("  → {:#?}", error);
                exit(1);
            }
        }
    };

    let images = match get_images(&input) {
        Ok(images) => images,
        Err(message) => {
            cprintln!("{}", message);
            exit(1);
        }
    };

    let mut dynamic_images: Vec<(&PathBuf,DynamicImage)> = Vec::new();

    for image in &images {
        match load_any_image(image, *width, *height) {
            Ok(output) => dynamic_images.push((image, output)),
            Err(error) => {
                let name = image.to_str().unwrap();
                cprintln!("<r>⨯ Error:</> loading: <b,u>{}</>", name);
                cprintln!("  → {:#?}", error);
                cprintln!("---");
            }
        }
    }

    for tuple in dynamic_images {
        let dynamic_image: DynamicImage = tuple.1;
        let input: &PathBuf = tuple.0;

        let output = match output.extension() {
            None => {
                let input_ = input.file_stem().unwrap().to_str().unwrap();
                output.join(format!("{}.{}", input_, &format))
            }
            Some(_) => {
                output.clone()
            }
        };

        let parent = output.parent().unwrap();
        fs::create_dir_all(parent).unwrap();

        let on_success = || {
            let input = input.as_os_str().to_str().unwrap();
            let output = output.as_path().to_str().unwrap();
            cprintln!("<g>✓ saved:</> {} -> {}", input, output);
        };

        let on_error = || {
            let input = input.as_os_str().to_str().unwrap();
            let output = output.as_path().to_str().unwrap();
            cprintln!("<r>⨯ error:</> {} -> {}", input, output);
        };

        match format.as_str() {
            "webp" => {
                match save_as_webp(dynamic_image, &output, *quality) {
                    Ok(_) => on_success(),
                    Err(error) => {
                        on_error();
                        eprintln!("  → {:#?}", error);
                    }
                }
            }
            _ => {
                let format = ImageFormat::from_extension(&format).unwrap();
                match save_as_format(dynamic_image, &output, format) {
                    Ok(_) => on_success(),
                    Err(error) => {
                        on_error();
                        eprintln!("  → {:#?}", error);
                    }
                }
            }
        }
    }
}