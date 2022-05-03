/// Marks the different types of Dithering Algorithms available
#[derive(Debug)]
pub enum DitherMethod {
    /// Grayscales the image + marks pixels as either Full Black or Full White
    BinaryQuantisation,
}

#[derive(Debug)]
pub struct Config {
    pub dithering_method: DitherMethod,
    pub source_filename: String,
    pub destination_filename: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough Arguments. Expected: 'sourceFilename' ['destFilename']");
        }

        let dithering_method = DitherMethod::BinaryQuantisation;

        let destination_filename = if args.len() == 3 {
            Some(args[2].clone())
        } else {
            None
        };

        let source_filename = args[1].clone();

        return Ok(Config {
            dithering_method,
            source_filename,
            destination_filename,
        });
    }
}