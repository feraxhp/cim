use clap::{arg, command, crate_description, Command};
use crate::formats::validate::input;

pub(crate) fn cim_command() -> Command {
    let parser = input::validate;
    command!()
        .about(crate_description!())
        .args([
            arg!(<format> "The desire format")
                .value_parser(parser)
            ,
        ])

}