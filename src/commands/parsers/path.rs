use std::{path::PathBuf, str::FromStr};

use color_print::cformat;

use crate::tools::paths::path_to_absolute;


pub fn path(value: &str) -> Result<PathBuf, String> {
    match PathBuf::from_str(value) {
        Ok(path_) => match path_to_absolute(&path_) {
            Ok(path_) => {
                Ok(path_)
            }
            Err(error) => Err(
                cformat!("<r>* Error: Could not determine the input path</>\n  <g>→</g> {}", error)
            )
        },
        Err(e) => Err(
            cformat!("<r>* Error: Could not determine the input path</>\n  <g>→</g> {}", e)
        )
    }
}
