use std::time::Duration;
use indicatif::ProgressBar;

use super::styles::Styles;

pub struct Spinner;


impl Spinner {
    pub fn sub_proccess<T: Into<String>>(message: T, spinner: ProgressBar) -> ProgressBar {
        spinner.set_style(Styles::sub_proccess());
        spinner.set_message(message.into());
        spinner.enable_steady_tick(Duration::from_millis(100));
        
        spinner
    }
    
    pub fn main_proccess<T: Into<String>>(message: T, spinner: ProgressBar) -> ProgressBar {
        spinner.set_style(Styles::main_proccess());
        spinner.set_message(message.into());
        spinner.enable_steady_tick(Duration::from_millis(80));
        
        spinner
    }
}
