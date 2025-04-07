use color_print::cformat;

const VALID_FORMATS: [&str; 21] = [
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
  /* ImageFormat::Qoi */  "qoi",
    "svg"
];

pub(crate) fn is_valid_format(value: &str) -> Result<String, String> {
    match is_supported_format(value) {
        true => Ok(value.to_string()),
        false => Err(
            cformat!(
                "\n<g>• Valid formats are:</g>\n  {}",
                VALID_FORMATS.join(", ")
            )
        ),
    }
}

pub(crate) fn is_supported_format(value: &str) -> bool {
    VALID_FORMATS.iter().any(|f| value == *f)
}