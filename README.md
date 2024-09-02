# CIM (CONVERT IMAGE)

A simple CLI tool written in Rust for easily converting between various image formats.

You need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system to build this project.

## supported formats

- Png,
- Jpeg,
- WebP,
- Gif,
- Pnm,
- Tiff,
- Tga,
- Dds,
- Bmp,
- Ico,
- Hdr,
- OpenExr,
- Farbfeld,
- Avif,
- Qoi,

### Note
- svg is only supported as input format.
  - it supports resizing. 
    - width and height are required.
- webp supports lossless and lossy compression.
  - quality is required.

## Usage

cim [format] <input> <output> [options]

Arguments:
  <format>  The desire file Format
  <input>   Input file/directory path
  [output]  Output file/directory path *(optional)

Options:
  -w, --width <value>    Width of the output image (only for SVG to image) [default: 0]
  -h, --height <value>   Height of the output image (only for SVG to image) [default: 0]
  -q, --quality <value>  Quality of the output image (only for image to WebP) [default: 100]
      --help             Prints help information
  -V, --version          Print version

## Build and Run
```bash

# clone the repository
git clone https://github.com/feraxhp/cim.git
cd cim

# build the project
cargo build --release

# if you want to install it on your system
cargo install --path .
```

## to do
- [ ] Create distribution packages
  - [ ] deb
  - [ ] rpm
  - [ ] pacman
  - [ ] Windows installer
  - [ ] macOS brew

- [ ] Add more image formats
  - [ ] pdf

- [ ] Add controls for image quality
  - [ ] jpeg
  - [ ] avif