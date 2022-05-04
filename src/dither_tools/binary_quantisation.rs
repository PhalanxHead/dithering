use image::{imageops, DynamicImage};
use rand::Rng;

/// Greyscales the image and marks each pixel as either Full Black or Full White
pub fn binary_quantisation(img: DynamicImage) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);
    for pixel in subimg.pixels_mut() {
        pixel[0] = if pixel[0] > 128 { 255 } else { 0 }
    }
    return DynamicImage::from(subimg);
}

/// Greyscales the image and marks each pixel as either Full Black or Full White, adding some random noise to the classifier
pub fn binary_quantisation_with_noise(img: DynamicImage) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);
    let mut rng = rand::thread_rng();
    for pixel in subimg.pixels_mut() {
        let noise_val: u16 = rng.gen_range(0..96) + (pixel[0] as u16);
        pixel[0] = if noise_val > 128 { 255 } else { 0 }
    }
    return DynamicImage::from(subimg);
}

/// Greyscales the image and marks each pixel as either Full Black or Full White based on a random threshold value.
pub fn white_noise_quantisation(img: DynamicImage) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);
    let mut rng = rand::thread_rng();
    for pixel in subimg.pixels_mut() {
        pixel[0] = if pixel[0] > rng.gen_range(0..255) {
            255
        } else {
            0
        }
    }
    return DynamicImage::from(subimg);
}
