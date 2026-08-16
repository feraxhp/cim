use image::DynamicImage;
// use image::GenericImageView;
use image::imageops::FilterType;


#[derive(Debug, Clone)]
pub struct Size {
    height: u32,
    width: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self { Self { height, width } }
    
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    
    // pub fn default(image: DynamicImage) -> Self {
    //     let dimentions = image.dimensions();
        
    //     Self { height: dimentions.1, width: dimentions.0 }
    // }
}

pub fn resize(image: DynamicImage, size: Size) -> DynamicImage {
    image.resize(size.width(), size.height(), FilterType::Lanczos3)
}


