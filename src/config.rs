use clap::{ArgEnum, Parser};
use std::path::PathBuf;

/// Marks the different types of Dithering Algorithms available
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum DitherMethod {
    /// Converts the image to greyscale
    GreyscaleRaw,
    /// Greyscales the image + marks pixels as either Full Black or Full White
    BinaryQuantisation,
    /// Greyscales the image + marks pixels as either black or white, with some random noise added to the classifier
    NoisyQuantisation,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// The dithering algorithm to use to adjust the image
    #[clap(arg_enum, default_value_t=DitherMethod::GreyscaleRaw)]
    pub dithering_method: DitherMethod,
    /// The path (absolute or relative) + filename + extension of the source image
    #[clap(short, long, parse(from_os_str))]
    pub source_filename: PathBuf,
    /// The path (absolute or relative) + filename + extension to write the altered image to
    #[clap(short, long, parse(from_os_str))]
    pub destination_filename: Option<PathBuf>,
}
