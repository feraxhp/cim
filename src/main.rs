mod conversion;
mod tools;

use tools::*;

use clap::builder::{TypedValueParser, ValueParser};
use clap::{arg, command, value_parser, ArgAction};
use color_print::{cformat, cprintln};
use std::any::Any;
use std::error::Error;
use std::ops::Add;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs, path};
use std::env::current_dir;
use std::ffi::OsStr;
use image::{math, DynamicImage, ImageFormat};
use crate::conversion::load_any_image::load_any_image;
use crate::conversion::save_as_format::save_as_format;
use crate::conversion::save_as_webp::save_as_webp;
use crate::tools::files::get_images;
use path_absolutize::*;
use crate::tools::paths::path_to_absolute;

fn main() {
    let conversion = command!()
        .name("cim (convert image)")
        .about("Convert images to different formats")
        .disable_help_flag(true)
        .arg(
            arg!(<format> "The desire file Format")
                .value_parser(format::is_valid_format)
        )
        .arg(arg!(<input> "Input file/directory path").id("input")
            .required(true)
            .value_hint(clap::ValueHint::DirPath)
            .value_parser(ValueParser::path_buf())
        )
        .arg(arg!(<output> "Output file/directory path *(optional)")
            .required(false)
            .value_hint(clap::ValueHint::DirPath)
            .value_parser(ValueParser::path_buf())
        )
        .arg(arg!(-w --width <value> "Width of the output image (only for SVG to image)")
            .required(false)
            .default_value("0")
            .value_parser(value_parser!(u32))
        )
        .arg(arg!(-h --height <value> "Height of the output image (only for SVG to image)")
            .required(false)
            .default_value("0")
            .value_parser(value_parser!(u32))
        )
        .arg(arg!(-q --quality <value> "Quality of the output image (only for image to WebP)")
            .required(false)
            .default_value("100")
            .value_parser(value_parser!(f32))
        )
        .arg(arg!(   --help "Prints help information").action(ArgAction::Help))
        .get_matches();

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
    // let mut errors: Vec<String> = Vec::new();

    for image in &images {
        match load_any_image(image, *width, *height) {
            Ok(output) => dynamic_images.push((image, output)),
            Err(error) => {
                let name = image.file_name().unwrap().to_str().unwrap();
                // errors.push(cformat!("Image {}, couldn't be load", name, error));
                cprintln!("<r>⨯</> Error loading: {} | {}", name, error);
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