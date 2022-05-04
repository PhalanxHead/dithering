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

```bash
cargo run -s ./LukaTest.jpg noisy-quatisation
cargo run -s ./LukaTest.jpg bayer-ordered-dithering one
```

Available algorithms:

- Raw Greyscale (`greyscale-raw`): Turns the image greyscale using `image.rs` greyscale function.
- Binary Quantisation (`binary-quantisation`): Turns the image into full black + white pixels based on their brightness. (Brightness >= 0.5 become white)
- Noisy Quantisation (`noisy-quantisation`): Turns the image into a set of full black + white pixels based on their brightness plus some random value.
- White Noise Quantisation (`white-noise-quantisation`): Turns the image into a set of full black + white pixels based on their brightness measured against some random threshold.
- Bayer Ordered Dithering (`bayer-ordered-dithering <level>`): Turns the image into a set of full black + white pixels based on thresholds determined by a pre-computed [Bayer Matrix](https://en.wikipedia.org/wiki/Ordered_dithering#cite_note-bayermatrix-1)
  - The level indicates the size of the bayer matrix to be used. `zero` indicates a 2x2 matrix, `one` indicates a 4x4 matrix, `two` indicates an 8x8 matrix, etc


## Building

```bash
cargo build --release
```



## Help Listing

After building, the program can generate help information (it uses the `clap` crate to do so).

```
./dithering --help
./dithering help bayer-ordered-dithering
```

