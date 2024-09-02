mod conversion;

use crate::conversion::load_any_image::load_any_image;
use crate::conversion::save_as_format::save_as_format;
use crate::conversion::save_as_webp::save_as_webp;
use clap::builder::TypedValueParser;
use clap::{arg, command, value_parser, ArgAction};
use image::{DynamicImage, ImageFormat};
use std::any::Any;
use std::error::Error;
use std::fs::metadata;
use std::ops::Add;
use std::path::Path;
use std::{env, fs};
use resvg::usvg::filter::Image;

fn main() {
    let conversion = command!()
        .name("cim (convert image)")
        .about("Convert images to different formats")
        .disable_help_flag(true)
        .arg(arg!(<format> "The desire file Format"))
        .arg(arg!(<input> "Input file/directory path")
            .required(true)
            .id("input")
        )
        .arg(arg!(<output> "Output file/directory path *(optional)")
            .required(false)
            .default_value("None")
            .hide_default_value(true)
            .id("output")
        )
        .arg(arg!(-w --width "Width of the output image (only for SVG to image)")
            .required(false)
            .default_value("0")
            .value_parser(value_parser!(u32))
        )
        .arg(arg!(-h --height "Height of the output image (only for SVG to image)")
            .required(false)
            .default_value("0")
            .value_parser(value_parser!(u32))
        )
        .arg(arg!(-q --quality "Quality of the output image (only for image to WebP)")
            .required(false)
            .default_value("100")
            .value_parser(value_parser!(f32))
        )
        .arg(arg!(   --help "Prints help information").action(ArgAction::Help))
        .get_matches();

    let format = conversion.get_one::<String>("format").unwrap().to_lowercase();
    let input = conversion.get_one::<String>("input").unwrap().to_lowercase();
    let output = conversion.get_one::<String>("output").unwrap().to_lowercase();
    let width = conversion.get_one::<u32>("width").unwrap();
    let height = conversion.get_one::<u32>("height").unwrap();
    let quality = conversion.get_one::<f32>("quality").unwrap();

    println!("output: {output}");

    let input_path = metadata(input.clone()).unwrap();
    let input_is_file = input_path.is_file();
    let images: Vec<(String, DynamicImage)> = match input_is_file {
        true => {
            let input_clone = input.clone();
            vec![
                (input.clone(), load_any_image(&input_clone, width.clone(), height.clone()).unwrap())
            ]
        }
        false => {
            let files = get_files(&input);
            let mut images: Vec<(String, DynamicImage)> = Vec::new();
            for file in files {
                let file_clone = file.clone();
                let file_str = file.clone();
                match load_any_image(&file_clone, width.clone(), height.clone()) {
                    Ok(image) => images.append(&mut vec![(file_str, image)]),
                    Err(e) => {
                        eprintln!("Error loading {file}: {e}");
                        continue;
                    },
                }
            };
            images
        }
    };

    let outputs = match output.as_str() {
        "None" => {
            let mut outputs: Vec<String> = Vec::new();
            for image in images.clone() {
                let mut output_path = image.0.replace(".", "-");
                output_path.push_str(".");
                output_path.push_str(&format);
                outputs.append(&mut vec![output_path]);
            }
            outputs
        },
        "." => {
            if input_is_file {
                vec![get_path(".".to_string(), &format, input)]
            } else {
                get_paths(output, &format, images.iter().map(|image| image.0.clone()).collect())
            }
        },
        _ => {
            let output_is_file = output.clone().contains(".");
            if input_is_file && output_is_file { vec![output] }
            else if !input_is_file && !output_is_file {
                get_paths(output, &format, images.iter().map(|image| image.0.clone()).collect())
            } else if input_is_file {
                vec![get_path(output, &format, input)]
            } else {
                panic!("Output must be a directory if input is a directory");
            }
        }
    };

    assert_eq!(images.len(), outputs.len());

    match format.as_str() {
        "svg" => {
            panic!("SVG is not supported as an output format");
        },
        "webp" => {
            for (image, output) in images.iter().zip(outputs.iter()) {
                let path = image.0.split("/").last().unwrap();
                let dimage = image.clone().1;
                match save_as_webp(dimage, output, quality.clone()) {
                    Ok(_) => {
                        println!("Image {path} converted successfully");
                    },
                    Err(e) => {
                        eprintln!("Error while converting {path}: {e}");
                    },
                }
            }
        },
        _ => {
            for (image, output) in images.iter().zip(outputs.iter()) {
                let path = image.0.split("/").last().unwrap();
                let dimage = image.clone().1;
                let ext = ImageFormat::from_extension(format.as_str()).unwrap();
                match save_as_format(dimage, output, ext) {
                    Ok(_) => {
                        println!("Image {path} converted successfully");
                    },
                    Err(e) => {
                        eprintln!("Error while converting {path}: {e}");
                    },
                }
            }
        }
    }
}


fn get_files(dir: &str) -> Vec<String> {
    match fs::read_dir(Path::new(dir)) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                if entry.file_type().ok()?.is_file() { entry.path().to_str().map(|s| s.to_string())
                } else { None }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn get_paths(dir: String, format: &str, images: Vec<String>) -> Vec<String> {
    let mut outputs: Vec<String> = Vec::new();
    for image in images.clone() {
        let mut output_path = dir.clone();
        let filename = image.split("/").last().unwrap();
        output_path.push_str("/");
        output_path.push_str(&filename.replace(".", "-"));
        output_path.push_str(".");
        output_path.push_str(&format);
        outputs.append(&mut vec![output_path]);
    }

    for o in outputs.clone() {
        println!("{o}");
    }

    outputs
}

fn get_path(path: String, format: &str, base: String) -> String {
    let mut output_path = path.clone();
    let input_file = base.split("/").last().unwrap();
    let input_file = input_file.split('.').collect::<Vec<&str>>();
    let input_file_name = input_file[0];

    output_path.push_str("/");
    output_path.push_str(input_file_name);
    output_path.push_str(".");
    output_path.push_str(&format);

    output_path
}