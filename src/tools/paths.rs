use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use path_absolutize::Absolutize;

pub(crate) fn path_to_absolute(path: &PathBuf) -> Result<PathBuf, String> {
    if path.is_absolute() { return Ok(path.clone()); }

    let path_str = path.to_string_lossy();
    if path_str.starts_with("~/") || path_str == "~" {
        return if let Ok(home_dir) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
            let home_path = PathBuf::from(home_dir);

            if path_str == "~" { return Ok(home_path); }

            let remainder = path_str.strip_prefix("~/").unwrap();
            let remainder_path = PathBuf::from(remainder);

            match remainder_path.absolutize_from(home_path) {
                Ok(path) => { Ok(path.to_path_buf()) }
                Err(error) => Err(format!("{:#?}", error))
            }
        } else {
            Err("Could not determine the home directory".to_string())
        }
    };

    match current_dir() {
        Ok(current) => {
            match path.absolutize_from(current) {
                Ok(path) => { Ok(path.to_path_buf()) }
                Err(error) => Err(format!("{:#?}", error))
            }
        }
        Err(error) => Err(format!("{:#?}", error))
    }
}