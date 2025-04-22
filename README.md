# CIM (CONVERT IMAGE)

A simple CLI tool written in Rust for easily converting between various image formats.

## Supported output formats

| Png      | Jpeg     |
| -------- | -------- |
| WebP     | Gif      |
| Pnm      | Tiff     |
| Tga      | Dds      |
| Bmp      | Ico      |
| Hdr      | OpenExr  |
| Farbfeld | Avif     |
| Qoi      |          |

## Supported input formats

- All output formats
- SVG

> [!Note]
> `svg` is only supported as input format.
> And supports resizing.
> - _width_ and _height_ are required.

> [!Note]
> `webp` supports lossless and lossy compression.
>  - _quality_ is required.

## Usage
```bash
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
  -v, --vnumber          Prints the version number
  -V, --version          Print version
```

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
  - [x] deb
  - [x] rpm
  - [ ] pacman
  - [ ] Windows installer
  - [x] Windows exe
  - [ ] macOS brew

- [ ] Add more image formats
  - [ ] pdf

- [ ] Add controls for image quality
  - [ ] jpeg
  - [ ] avif
