mod conversion;

use crate::conversion::save_as_format::save_as_format;
use crate::conversion::load_image::load_image;
use crate::conversion::load_svg::load_svg;
use std::{env, fs};
use std::ops::Add;
use std::path::Path;
use std::error::Error;
use image::ImageFormat;

fn main() {

    for argument in env::args() {

    }

    // let converter = (image_to_webp, ".webp");
    // let converter = (image_to_jpg, ".jpg");
    // // let converter =(image_to_png, ".png");
    //
    // let files = get_files("images");
    //
    // for input in files.iter(){
    //     print!("{input}: ");
    //
    //     let output = input.replace(".", "-") + converter.1;
    //     let output = output.replace("images", "output");
    //
    //     let img = match load_image(input) {
    //         Ok(image) => image,
    //         Err(e) => {
    //             println!("{e}");
    //             continue
    //         }
    //     };
    //
    //     match converter.0(img, output.as_str(), 75.0) {
    //         Ok(_) => println!("Imagen convertida exitosamente."),
    //         Err(e) => println!("Error al convertir la imagen: {}", e),
    //     }
    // }

    match load_svg(
        "images/python-package.svg",
        "output.png",
        2024,
        2024
    ) {
        Ok(image) => match save_as_format(
            image,
            "output.jpg",
            ImageFormat::Jpeg
        ) {
            Ok(_) => println!("Imagen convertida exitosamente."),
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => eprintln!("Error: {}", e),
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
