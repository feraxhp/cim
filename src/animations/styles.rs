use color_print::cformat;
use indicatif::ProgressStyle;

pub struct  Styles;

impl Styles {
    pub fn main_proccess() -> ProgressStyle {
        ProgressStyle::default_spinner()
            .tick_strings(
                &[
                    cformat!("<g, bold>⢀⠀</>").as_str(),
                    cformat!("<g, bold>⡀⠀</>").as_str(),
                    cformat!("<g, bold>⠄⠀</>").as_str(),
                    cformat!("<g, bold>⢂⠀</>").as_str(),
                    cformat!("<g, bold>⡂⠀</>").as_str(),
                    cformat!("<g, bold>⠅⠀</>").as_str(),
                    cformat!("<g, bold>⢃⠀</>").as_str(),
                    cformat!("<g, bold>⡃⠀</>").as_str(),
                    cformat!("<g, bold>⠍⠀</>").as_str(),
                    cformat!("<g, bold>⢋⠀</>").as_str(),
                    cformat!("<g, bold>⡋⠀</>").as_str(),
                    cformat!("<g, bold>⠍⠁</>").as_str(),
                    cformat!("<g, bold>⢋⠁</>").as_str(),
                    cformat!("<g, bold>⡋⠁</>").as_str(),
                    cformat!("<g, bold>⠍⠉</>").as_str(),
                    cformat!("<g, bold>⠋⠉</>").as_str(),
                    cformat!("<g, bold>⠋⠉</>").as_str(),
                    cformat!("<g, bold>⠉⠙</>").as_str(),
                    cformat!("<g, bold>⠉⠙</>").as_str(),
                    cformat!("<g, bold>⠉⠩</>").as_str(),
                    cformat!("<g, bold>⠈⢙</>").as_str(),
                    cformat!("<g, bold>⠈⡙</>").as_str(),
                    cformat!("<g, bold>⢈⠩</>").as_str(),
                    cformat!("<g, bold>⡀⢙</>").as_str(),
                    cformat!("<g, bold>⠄⡙</>").as_str(),
                    cformat!("<g, bold>⢂⠩</>").as_str(),
                    cformat!("<g, bold>⡂⢘</>").as_str(),
                    cformat!("<g, bold>⠅⡘</>").as_str(),
                    cformat!("<g, bold>⢃⠨</>").as_str(),
                    cformat!("<g, bold>⡃⢐</>").as_str(),
                    cformat!("<g, bold>⠍⡐</>").as_str(),
                    cformat!("<g, bold>⢋⠠</>").as_str(),
                    cformat!("<g, bold>⡋⢀</>").as_str(),
                    cformat!("<g, bold>⠍⡁</>").as_str(),
                    cformat!("<g, bold>⢋⠁</>").as_str(),
                    cformat!("<g, bold>⡋⠁</>").as_str(),
                    cformat!("<g, bold>⠍⠉</>").as_str(),
                    cformat!("<g, bold>⠋⠉</>").as_str(),
                    cformat!("<g, bold>⠋⠉</>").as_str(),
                    cformat!("<g, bold>⠉⠙</>").as_str(),
                    cformat!("<g, bold>⠉⠙</>").as_str(),
                    cformat!("<g, bold>⠉⠩</>").as_str(),
                    cformat!("<g, bold>⠈⢙</>").as_str(),
                    cformat!("<g, bold>⠈⡙</>").as_str(),
                    cformat!("<g, bold>⠈⠩</>").as_str(),
                    cformat!("<g, bold>⠀⢙</>").as_str(),
                    cformat!("<g, bold>⠀⡙</>").as_str(),
                    cformat!("<g, bold>⠀⠩</>").as_str(),
                    cformat!("<g, bold>⠀⢘</>").as_str(),
                    cformat!("<g, bold>⠀⡘</>").as_str(),
                    cformat!("<g, bold>⠀⠨</>").as_str(),
                    cformat!("<g, bold>⠀⢐</>").as_str(),
                    cformat!("<g, bold>⠀⡐</>").as_str(),
                    cformat!("<g, bold>⠀⠠</>").as_str(),
                    cformat!("<g, bold>⠀⢀</>").as_str(),
                    cformat!("<g, bold>⠀⡀</>").as_str(),
                ]
            )
    }
    
    pub fn sub_proccess() -> ProgressStyle {
        ProgressStyle::default_spinner()
            .tick_strings(
                &[
                    cformat!("<g, bold> ◜</>").as_str(),
                    cformat!("<g, bold> ◠</>").as_str(),
                    cformat!("<g, bold> ◝</>").as_str(),
                    cformat!("<g, bold> ◞</>").as_str(),
                    cformat!("<g, bold> ◡</>").as_str(),
                    cformat!("<g, bold> ◟</>").as_str()
                ]
            )
    }
    
    pub fn success() -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(
            &[ cformat!("").as_str() ]
        )
    }
    
    pub fn success2() -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(
            &[ cformat!("<g> ✓</>").as_str() ]
        )
    }
    
    pub fn error() -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(
            &[ cformat!("<r> ✕</>").as_str() ]
        )
    }
}