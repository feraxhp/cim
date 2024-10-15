mod conversion;
mod commands;
mod formats;
mod open;
mod save;

use crate::commands::command::cim_command;
use crate::commands::manager::cim_manager;
use clap::builder::TypedValueParser;
use clap::{arg, crate_version};
use std::any::Any;
use std::error::Error;
use std::io::Write;
use std::ops::Add;
use std::process::exit;
use std::io;

fn main() {
    let commands = cim_command()
        .arg(arg!(-v --vnumber "Prints the version number to the standard output").exclusive(true))
        .get_matches();

    match commands.clone().args_present() {
        true => {
            if *commands.get_one::<bool>("vnumber").unwrap_or(&false) {
                let version = crate_version!();
                let _ = io::stdout().write(version.as_bytes());
                println!();
                exit(0);
            }
        },
        _ => {}
    }

    cim_manager(&commands);
}
