use image::GenericImageView;
use std::error::Error;

mod config;
pub use config::Config;
use config::DitherMethod;

mod dither_tools;

pub fn run(args: Config) -> Result<(), Box<dyn Error>> {
    // Write the contents of this image to the Writer in PNG format.
    let dest_name = match &args.destination_filename {
        None => "test.png",
        Some(x) => x.to_str().unwrap(),
    };

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(args.source_filename)?;

    println!("Input Dimensions {:?}", img.dimensions());
    println!("Input ImgColour: {:?}", img.color());

    let subimg = match args.dithering_method {
        DitherMethod::GreyscaleRaw => dither_tools::greyscale(img),
        DitherMethod::BinaryQuantisation => dither_tools::binary_quantisation(img),
        DitherMethod::NoisyQuantisation => dither_tools::binary_quantisation_with_noise(img),
        DitherMethod::WhiteNoiseQuantisation => dither_tools::white_noise_quantisation(img),
        DitherMethod::BayerOrderedDithering { level } => dither_tools::bayer_ordered_dithering(img, level),
    };

    println!("Output Dimensions {:?}", subimg.dimensions());
    println!("Output ImgColour: {:?}", subimg.color());
    println!("Writing output image to {}", dest_name);
    subimg.save(dest_name)?;

    return Ok(());
}
