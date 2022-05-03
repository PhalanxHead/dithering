# Dithering

A small package that implements a series of dithering algorithms to manipulate photos.

## How to use

### Specify only source image

```bash
# cargo run -s '<path to source image (relative or absolute) including file extension>'
cargo run -s './LukaTest.jpg'
```

### Specify source image and destination

```bash
# cargo run -s '<path to source image>' -d '<path to destination file (including file extension)>'
cargo run -s './LukaTest.jpg' -d './output.png'
```

### Specify dithering algorithms

Available algorithms:

- Raw Greyscale (`greyscale-raw`): Turns the image greyscale using `image.rs` greyscale function.
- Binary Quantisation (`binary-quantisation`): Turns the image into full black + white pixels based on their brightness. (Brightness >= 0.5 become white)
- Noisy Quantisation (`noisy-quantisation`): Turns the image into a set of full black + white pixels based on their brightness plus some random value.

## Building

```bash
cargo build --release
```
