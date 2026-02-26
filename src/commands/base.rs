use clap::{arg, command, value_parser, ArgAction, Command};

use crate::commands::parsers::version::show_version;
use crate::tools::format::{self, VALID_FORMATS};
use crate::commands::parsers::path::path;


pub fn command() -> Command {
    command!().name("cim (convert image)")
            .about("Convert images to different formats")
            .disable_help_flag(true)
            .args([
                arg!(<format> "The desire file Format")
                    .value_names(VALID_FORMATS)
                    .value_parser(format::is_valid_format)
                ,
                arg!(-c --concurrent [concurrent] "The number of concurrent operations")
                    .default_value("4")
                    .value_parser(value_parser!(usize))
                ,
                arg!(<input> "Input file/directory path").id("input")
                    .value_hint(clap::ValueHint::DirPath)
                    .value_parser(path)
                ,
                arg!([output] "Output file/directory path *(optional)")
                    .value_hint(clap::ValueHint::DirPath)
                    .value_parser(path)
                ,
                arg!(-w --width <value> "Width of the output image (only for SVG to image)")
                    .required(false)
                    .default_value("32")
                    .value_parser(value_parser!(u32))
                ,
                arg!(-h --height <value> "Height of the output image (only for SVG to image)")
                    .required(false)
                    .default_value("32")
                    .value_parser(value_parser!(u32))
                ,
                arg!(-q --quality <value> "Quality of the output image (only for image to WebP)")
                    .required(false)
                    .default_value("100")
                    .value_parser(value_parser!(f32))
                ,
                arg!(-v --vnumber "Prints the version number to the standard output")
                    .exclusive(true)
                    .value_parser(show_version)
                ,
                arg!(   --help "Prints help information").action(ArgAction::Help)
            ])
}
