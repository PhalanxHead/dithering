use clap::{Subcommand, Parser};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
pub enum DitherMethod {
    /// Converts the image to greyscale
    GreyscaleRaw,
    /// Greyscales the image + marks pixels as either Full Black or Full White
    BinaryQuantisation,
    /// Greyscales the image + marks pixels as either black or white, with some random noise added to the classifier
    NoisyQuantisation,
    /// Greyscales the image + marks pixels as either black or white, using a pseudorandom threshold
    WhiteNoiseQuantisation,
    /// Greyscales, then uses Bayer Ordered Dithering to determine the black/white value of a pixel
    BayerOrderedDithering { 
        #[clap(arg_enum, default_value_t=crate::dither_tools::BayerLevel::Zero)]
        level: crate::dither_tools::BayerLevel 
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// The dithering algorithm to use to adjust the image
    #[clap(subcommand)]
    pub dithering_method: DitherMethod,
    /// The path (absolute or relative) + filename + extension of the source image
    #[clap(short, long, parse(from_os_str))]
    pub source_filename: PathBuf,
    /// The path (absolute or relative) + filename + extension to write the altered image to
    #[clap(short, long, parse(from_os_str))]
    pub destination_filename: Option<PathBuf>,
}
