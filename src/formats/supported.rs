use color_print::cformat;

const SUPPORTED_COMMON_FORMATS: [&str; 15] = [
    "png",
    "jpeg",
    "webp",
    "gif",
    "pnm",
    "tiff",
    "tga",
    "dds",
    "bmp",
    "ico",
    "hdr",
    "openExr",
    "farbfeld",
    "avif",
    "qoi",
];

pub(crate) fn SUPPORTED_INPUT_FORMATS() -> Vec<&'static str> {

    let mut supported = SUPPORTED_COMMON_FORMATS.to_vec();

    supported.append(&mut vec![
        "svg"
    ]);

    supported
}

pub(crate) fn SUPPORTED_OUTPUT_FORMATS()-> Vec<&'static str> {
    let mut supported = SUPPORTED_COMMON_FORMATS.to_vec();

    // supported.append(&mut vec![
    //     "svg"
    // ]);

    supported
}
