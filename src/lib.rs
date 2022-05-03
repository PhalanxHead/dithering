use clap::Parser;
use image::{imageops, DynamicImage, GenericImageView};
use std::error::Error;
use rand::Rng;

mod config;
use config::Config;
use config::DitherMethod;

pub fn run() -> Result<(), Box<dyn Error>> {
    let parsed_args = Config::parse();
    println!("{:?}", parsed_args);

    // Write the contents of this image to the Writer in PNG format.
    let dest_name = match &parsed_args.destination_filename {
        None => "test.png",
        Some(x) => x.to_str().unwrap(),
    };

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(parsed_args.source_filename)?;

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("ImgColour: {:?}", img.color());

    let subimg = match parsed_args.dithering_method {
        DitherMethod::GreyscaleRaw => DynamicImage::from(imageops::grayscale_alpha(&img)),
        DitherMethod::BinaryQuantisation => binary_quantisation(img),
        DitherMethod::NoisyQuantisation => binary_quantisation_with_noise(img),
    };

    println!("Writing output image to {}", dest_name);
    subimg.save(dest_name)?;

    return Ok(());
}

/// Greyscales the image and marks each pixel as either Full Black or Full White
fn binary_quantisation(img: DynamicImage) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);
    for pixel in subimg.pixels_mut() {
        pixel[0] = if pixel[0] > 128 { 255 } else { 0 }
    }
    return DynamicImage::from(subimg);
}

/// Greyscales the image and marks each pixel as either Full Black or Full White, adding some random noise to the classifier
fn binary_quantisation_with_noise(img: DynamicImage) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);
    let mut rng = rand::thread_rng();
    for pixel in subimg.pixels_mut() {
        let noise_val: u16 = rng.gen_range(0..96) + (pixel[0] as u16);
        pixel[0] = if noise_val > 128 { 255 } else { 0 }
    }
    return DynamicImage::from(subimg);
}