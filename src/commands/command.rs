use clap::{arg, command, crate_description, value_parser, Arg, ArgAction, ArgMatches, Command};
use clap::builder::{PathBufValueParser, ValueParser};
use color_print::cformat;
use crate::formats::supported::SUPPORTED_INPUT_FORMATS;
use crate::formats::validate::input;

pub(crate) fn cim_command() -> Command {
    let parser = input::validate;
    command!()
        .about(crate_description!())
        .disable_help_flag(true)
        .args([
            arg!(<format> "Desired format for the converted image.")
                .value_parser(parser)
            ,
            arg!(<input> "Path to the image or directory")
                .value_parser()
            ,
            arg!([output] "Output path to the image or directory")
            ,
            dimension_arg("height", 'h'),
            dimension_arg("width", 'w'),
            arg!(--help "Prints this!").action(ArgAction::HelpLong)
        ])
}

fn dimension_arg(id: &'static str, short: char) -> Arg {
    Arg::new(id.clone())
        .short(short.clone())
        .help(cformat!("The <g,b>{}</> dimension for <r,i>svg conversion</>", id.clone()))
        .default_value("0")
        .required(false)
        .value_parser(value_parser!(u32))
        .hide_default_value(true)
}
