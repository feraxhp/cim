mod conversion;
mod tools;

use tools::*;

use clap::builder::{TypedValueParser, ValueParser};
use clap::{arg, command, value_parser, ArgAction};
use color_print::cprintln;
use std::any::Any;
use std::error::Error;
use std::ops::Add;
use std::path::PathBuf;
use std::process::exit;
use std::env;
use crate::tools::files::get_images;

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
        Some(path) => path,
        None => {
            cprintln!("<r>* Error: Could not determine input path</>");
            exit(1);
        }
    };

    let output = match conversion.get_one::<PathBuf>("output") {
        Some(path) => path.clone(),
        None => env::current_dir().unwrap()
    };

    let images = match get_images(input) {
        Ok(images) => images,
        Err(message) => {
            cprintln!("{}", message);
            exit(1);
        }
    };

}