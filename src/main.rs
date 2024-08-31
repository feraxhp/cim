mod conversion;

use crate::conversion::image_to_jpg::image_to_jpg;
use crate::conversion::load_image::load_image;
use std::{env, fs};
use std::ops::Add;
use std::path::Path;
use std::error::Error;

fn main() {

    for argument in env::args() {
        if argument == "--help" {
            println!("You passed --help as one of the arguments!");
        }
    }

    // let converter = (image_to_webp, ".webp");
    let converter = (image_to_jpg, ".jpg");
    // let converter =(image_to_png, ".png");

    let files = get_files("images");

    for input in files.iter(){
        print!("{input}: ");

        let output = input.replace(".", "-") + converter.1;
        let output = output.replace("images", "output");

        let img = match load_image(input) {
            Ok(image) => image,
            Err(e) => {
                println!("{e}");
                continue
            }
        };

        match converter.0(img, output.as_str(), 75.0) {
            Ok(_) => println!("Imagen convertida exitosamente."),
            Err(e) => println!("Error al convertir la imagen: {}", e),
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
