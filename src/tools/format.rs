use color_print::cformat;

pub const VALID_FORMATS: [&str; 20] = [
  /* ImageFormat::Png */  "png",
  /* ImageFormat::Jpeg */  "jpg", "jpeg",
  /* ImageFormat::Gif */  "gif",
  /* ImageFormat::WebP */  "webp",
  /* ImageFormat::Pnm */  "pnm", "pbm", "pgm", "ppm",
  /* ImageFormat::Tiff */  "tiff", "tif",
  /* ImageFormat::Tga */  "tga",
  /* ImageFormat::Dds */  "dds",
  /* ImageFormat::Bmp */  "bmp",
  /* ImageFormat::Ico */  "ico",
  /* ImageFormat::Hdr */  "hdr",
  /* ImageFormat::OpenExr */  "exr",
  /* ImageFormat::Farbfeld */  "farbfeld",
  /* ImageFormat::Avif */  "avif",
  /* ImageFormat::Qoi */  "qoi"
];

const ADDITIONAL_FORMATS: [&str; 1] = ["svg"];

pub(crate) fn is_valid_format(value: &str) -> Result<String, String> {
    match is_supported_format(value) {
        true => Ok(value.to_string()),
        false => Err(
            if is_read_supported_format(value) {
                cformat!(
                    "\n* This format is only supported as input\n<g>• Valid formats are:</g>\n  {}",
                    VALID_FORMATS.join(", ")
                )
            } else {
                cformat!(
                    "\n<g>• Valid formats are:</g>\n  {}",
                    VALID_FORMATS.join(", ")
                )
            }
        ),
    }
}

pub(crate) fn is_supported_format(value: &str) -> bool {
    VALID_FORMATS.iter().any(|f| value == *f)
}

pub(crate) fn is_read_supported_format(value: &str) -> bool {
    ADDITIONAL_FORMATS.iter().any(|f| value == *f) || is_supported_format(value)
}