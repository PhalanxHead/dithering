use clap::ArgEnum;
use image::{imageops, DynamicImage};
use morton_encoding::morton_encode;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, ArgEnum)]
pub enum BayerLevel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

pub fn bayer_ordered_dithering(img: DynamicImage, level: BayerLevel) -> DynamicImage {
    let mut subimg = imageops::grayscale(&img);

    let bayer_matrix = generate_bayer_matrix(level);

    for (index, pixel) in subimg.pixels_mut().enumerate() {
        pixel[0] = if (pixel[0]) > bayer_matrix[index % bayer_matrix.len()] {
            255
        } else {
            0
        }
    }
    return DynamicImage::from(subimg);
}

fn generate_bayer_matrix(level: BayerLevel) -> Vec<u8> {
    let matrix_size = 2u8.pow((level as u32) + 1);
    let mut bayer_matrix: Vec<u8> = [].to_vec();
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            bayer_matrix.push(generate_bayer_threshold_value(i, j, level));
        }
    }

    return bayer_matrix;
}

fn generate_bayer_threshold_value(i: u8, j: u8, level: BayerLevel) -> u8 {
    // From Wikipedia: https://en.wikipedia.org/wiki/Ordered_dithering#Pre-calculated_threshold_maps
    //  M(i, j) = bit_reverse(bit_interleave(bitwise_xor(i, j), i)) / n ^ 2

    // n is the size of the bayer matrix
    let n = 2u8.pow((level as u32) + 1);
    let i_mod = i % n;
    let j_mod = j % n;
    //println!("n: {}", n);

    // Bitwise XOR the two index values
    let x = i_mod ^ j_mod;
    //println!("XORing i: {}, j: {}: {:#b}", i, j, x);
    // Interleave the bits of the xor'd number and the i index
    let interleaved = morton_encode([x, i_mod]);

    //println!("Interleaved value: {:#b}", interleaved);
    // Reverse the bits from the interleave operation
    // let reversed = interleaved.reverse_bits();
    // println!("Reversed value: {:#b}", reversed);
    // Divide the output by n^2
    // let quotient = n.pow(6);
    // let divided = (reversed as f32) / (quotient as f32);
    // println!("Dividing {} by {} gives value: {}", interleaved, quotient, divided);

    let multipler = match level {
        BayerLevel::Zero => 64,
        BayerLevel::One => 16,
        BayerLevel::Two => 4,
        BayerLevel::Three => 1,
    };

    return (interleaved * multipler) as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morton_encode_gives_double_length_integer_when_called_with_2_2_u8() {
        let u1 = 2u8; // 0 0 0 0 0 0 1 0
        let u2 = 2u8; //  0 0 0 0 0 0 1 0
        let encoded = morton_encode([u1, u2]); // 0000000000001100
        assert_eq!(encoded, 12u16);
    }

    #[test]
    fn morton_enc_gives_u16_where_first_input_number_leads() {
        let u1 = 11u8; // _0 0 0 0 1 0 1 1
        let u2 = 2u8; // _ 0 0 0 0 0 0 1 0
        let encoded_1lead = morton_encode([u1, u2]); //  0000000010001110
        let encoded_2lead = morton_encode([u2, u1]); // 0000000001001101
        assert_eq!(encoded_1lead, 142u16);
        assert_eq!(encoded_2lead, 77u16);
    }

    #[test]
    fn morton_enc_gives_16bit_int_with_bigendian_100_2() {
        let u1 = 100u8; // 0 1 1 0 0 1 0 0
        let u2 = 2u8; //  0 0 0 0 0 0 1 0
        let encoded = morton_encode([u1, u2]); // 0010100000100100
        assert_eq!(encoded, 10276u16);
    }

    #[test]
    fn bayer_generate_l0_works() {
        println!("{:?}", generate_bayer_matrix(BayerLevel::Zero));
    }

    #[test]
    fn bayer_generate_l1_works() {
        println!("{:?}", generate_bayer_matrix(BayerLevel::One));
    }
}
