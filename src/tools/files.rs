use std::fs;
use std::path::PathBuf;
use color_print::cformat;
use crate::tools::format::is_read_supported_format;

fn get_images_from_dir(dir: &PathBuf) -> Vec<PathBuf> {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                if !entry.file_type().ok()?.is_file() { return None; }
                if let Some(extension) = entry.path().extension() {
                    if (is_read_supported_format(extension.to_str().unwrap())) {
                        return Some(entry.path())
                    }
                }

                None
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub(crate) fn get_images(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
    if path.is_dir() {
        return Ok(get_images_from_dir(path));
    }

    if is_read_supported_format(path.extension().unwrap().to_str().unwrap()) {
        Ok(vec![path.clone()])
    } else {
        let name = path.file_stem().unwrap().to_str().unwrap();
        let extension = path.extension().unwrap().to_str().unwrap();
        Err(
            cformat!(
                "<r>* Error:</> <i>{}.<r>{}</> is not a supported format",
                name, extension
            )
        )
    }
}
