use color_print::cformat;

use crate::conversion::utils::rezise::Size;


impl Size {
    pub fn parse(value: &str) -> Result<Size, String> {
        let value = value.to_lowercase();
        if !value.contains("x") {
            return Err(cformat!("Invalid size, the correct format is <b>{{width}}</><r>x</><b>{{heigth}}</>"));
        }
        
        let parts: Vec<&str> = value.split('x').collect();
        if parts.len() > 2 || parts.len() < 2 {
            return Err(cformat!("Invalid size, the correct format is <b>{{width}}</><r>x</><b>{{heigth}}</>"));
        }
        
        let width: u32 = match parts[0].parse() {
            Ok(w) => w,
            Err(e) => {
                return Err(cformat!("{e}"));
            },
        };
        let height: u32 = match parts[0].parse() {
            Ok(w) => w,
            Err(e) => {
                return Err(cformat!("{e}"));
            },
        };
        
        Ok(Self::new(width, height))
    }
}
