# Dithering

A small package that implements a series of dithering algorithms to manipulate photos.

## How to use

### Specify only source image

```bash
# cargo run '<path to source image (relative or absolute) including file extension>'
cargo run './LukaTest.jpg'
```

### Specify source image and destination

```bash
# cargo run '<path to source image>' '<path to destination file (including file extension)>'
cargo run './LukaTest.jpg' './output.png'
```

## Building

```bash
cargo build --release
```
