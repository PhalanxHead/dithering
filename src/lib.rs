use image::GenericImageView;
use std::error::Error;

mod config;
use config::Config;
use config::DitherMethod;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let parsed_args = Config::new(args)?;
    println!("{:?}", parsed_args);

    // Write the contents of this image to the Writer in PNG format.
    let dest_name = match &parsed_args.destination_filename {
        None => "test.png",
        Some(x) => x,
    };

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(parsed_args.source_filename)?;

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("ImgColour: {:?}", img.color());

    let subimg = match parsed_args.dithering_method {
        DitherMethod::BinaryQuantisation => binary_quantisation(img),
    };

    println!("Writing output image to {}", dest_name);
    subimg.save(dest_name)?;

    return Ok(());
}

/// Greyscales the image and marks each pixel as either Full Black or Full White
fn binary_quantisation(img: image::DynamicImage) -> image::DynamicImage {
    let mut subimg = image::imageops::grayscale(&img);
    for pixel in subimg.pixels_mut() {
        pixel[0] = if pixel[0] > 128 { 255 } else { 0 }
    }
    return image::DynamicImage::from(subimg);
}
