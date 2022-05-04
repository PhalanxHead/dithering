use image::{imageops, DynamicImage};

mod binary_quantisation;
mod bayer_dithering;
pub use binary_quantisation::*;
pub use bayer_dithering::*;

/// Uses `image.rs`'s greyscale with alpha algorithm to produce a greyscale version of the input image
pub fn greyscale(img: DynamicImage) -> DynamicImage {
    let subimg = DynamicImage::from(imageops::grayscale_alpha(&img));
    return DynamicImage::from(subimg);
}
