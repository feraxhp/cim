mod conversion;
mod animations;
mod commands;
mod tools;

use color_print::{cformat, cprintln};
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::{env};
use indicatif::{MultiProgress, ProgressBar};

use crate::animations::spinners::Spinner;
use crate::animations::styles::Styles;
use crate::conversion::main::{convert, Options};
use crate::tools::files::get_images;


#[tokio::main]
async fn main() {
    let mprogres = MultiProgress::new();
    let progress = mprogres.add(ProgressBar::new_spinner());
    
    let conversion = commands::base::command().get_matches();
    let progress = Spinner::main_proccess("Strating...", progress);

    let options = Options {
        format: conversion.get_one::<String>("format").unwrap().to_lowercase(),
        width: conversion.get_one::<u32>("width").unwrap().to_owned(),
        height: conversion.get_one::<u32>("height").unwrap().to_owned(),
        quality: conversion.get_one::<f32>("quality").unwrap().to_owned()
    };
    
    let concurrent = conversion.get_one::<usize>("concurrent").unwrap().to_owned();

    let input = conversion.get_one::<PathBuf>("input").unwrap().clone();
    let output = match conversion.get_one::<PathBuf>("output") {
        Some(path_) => path_.clone(),
        None => match env::current_dir() {
            Ok(path) => { path }
            Err(error) => {
                cprintln!("<r>* Error: Could not get the output path</>");
                cprintln!("  → {:#?}", error);
                exit(1);
            }
        }
    };

    progress.set_message("Loading images ...");
    let images = match get_images(&input) {
        Ok(images) => images,
        Err(message) => {
            cprintln!("{}", message);
            exit(1);
        }
    };
    
    let amound = images.len();
    let stream_de_tareas = stream::iter(0..amound);
    let errors: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    
    progress.set_message(cformat!("Conveting images <m>0</>/<g>{}</> :: <c>{}</>", amound, concurrent));
    stream_de_tareas.for_each_concurrent(concurrent, |i| {
        let image = images[i].clone();
        let name = image.clone().file_name().unwrap().to_str().unwrap().to_string();
        let output = output.clone();
        let options = options.clone();
        let errors = errors.clone();
        let mpr = &mprogres;
        
        progress.set_message(cformat!("Conveting images <m>{}</>/<g>{}</> :: <c>{}</>", i + 1, amound, concurrent));
        async move {
            let prog = mpr.add(ProgressBar::new_spinner());
            let prog = Spinner::sub_proccess(&name, prog);
            
            match convert(&image, &output, &options).await {
                Ok(o) => {
                    let msg = cformat!("{} -> <c>{}</>", name, o);
                    
                    prog.set_style(Styles::success());
                    prog.finish_with_message(msg);
                }
                Err(e) => {
                    let msg = cformat!("<r>{}</> ← <m>{}</>", e, &image.into_os_string().to_str().unwrap().to_string());
                    
                    let mut errors = errors.lock().unwrap();
                    *errors += 1;
                    prog.set_style(Styles::error());
                    prog.finish_with_message(msg);
                }
            };
        }
    }).await;
    
    progress.finish_and_clear();
    
    let final_errors = errors.lock().unwrap().clone();
    
    if amound < 2 { exit(0) }
    println!("\n");
    match final_errors {
        0 => {
            cprintln!("<g>* <y,i>Conversion</y,i> finish successfully</>");
            cprintln!("  <g>-> <c>{} <g>images converted</>", amound)
        }
        f if f == amound => {
            cprintln!("<r>* <m,i>Conversion</m,i> completly fails</>");
            cprintln!("  <r>-> <m>{} <r>fails conversions</>", amound)
        }
        f => {
            cprintln!("<y>* <m,i>Conversion</m,i> finish with errors</>");
            cprintln!("  <g>-> <c>{} <g>images converted</>", amound - f);
            cprintln!("  <r>-> <m>{} <r>fails conversions</>", f)
        }
    }
}
