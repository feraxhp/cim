use color_print::{cformat, cprint};
use strsim::levenshtein;
use crate::formats::supported::SUPPORTED_INPUT_FORMATS;

pub(crate) fn validate(value: &str) -> Result<String, String> {
    let value = value.to_string();
    let value = value.to_lowercase();

    let formats = SUPPORTED_INPUT_FORMATS();
    match formats.contains(&value.as_str()) {
        true => Ok(value),
        false => {
            let mut closest_word = None;
            let mut ldistance = usize::MAX;
            for format in formats.clone() {
                let distance = levenshtein(format, value.as_str());
                if distance < ldistance {
                    ldistance = distance;
                    closest_word = Some(format.clone());
                }
            }

            let closest_word = if ldistance <= 3 { closest_word }
            else { None };

            let mut string = "\n\n* Valid options are: \n\n".to_string();
            let max = formats.iter().map(|r| r.len()).max().unwrap();
            for format3 in formats.chunks(3) {
                for format in format3 {
                    let format = match (format, closest_word.clone()) {
                        (f, Some(w)) if f.to_string() == w.to_string() => cformat!("<g,i>{0: <width$}</>", f, width=max.clone()),
                        _ => format.to_string(),
                    };
                    string.push_str(format!(" • {0: <width$}", format, width=max).as_str())
                }
                string.push('\n')
            }

            if closest_word.is_some() {
                string.push_str(
                    cformat!("\nDid you mean <g,i>{}</>?", closest_word.unwrap()).as_str()
                )
            } else {
                string.remove(string.len()-1);
            }

            Err(string)
        }
    }
}
