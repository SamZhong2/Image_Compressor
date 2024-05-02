use csc411_arith::chroma_of_index;
use crate::block_packing::unpack_block;
use array2::Array2;

use crate::codec;
use codec::Vcs;

/// Reverts the quantization of a DCT coefficient
fn undo_quantize(coeff: i64) -> f32 {
    if coeff.abs() == 15 {
        coeff as f32 * 0.3 / 15.0
    } else {
        coeff as f32 / 50.0
    }
}

/// Reverts the quantization of DCT coefficients
pub fn undo_quantiza_dct(a_quan: u64, b_quan: i64, c_quan: i64, d_quan: i64) -> (f32, f32, f32, f32) {
    let a = a_quan as f32 / 511.0;
    let b = undo_quantize(b_quan);
    let c = undo_quantize(c_quan);
    let d = undo_quantize(d_quan);
    (a, b, c, d)
}

/// Reverts the DCT transform to obtain the original Vcs block
pub fn undo_dct(a: f32, b: f32, c: f32, d: f32) -> Array2<Vcs> {
    let y1 = a - b - c + d;
    let y2 = a - b + c - d;
    let y3 = a + b - c - d;
    let y4 = a + b + c + d;
    Array2::from_row_major(2, 2, &vec![
        Vcs { y: y1, pb: 0.0, pr: 0.0 },
        Vcs { y: y2, pb: 0.0, pr: 0.0 },
        Vcs { y: y3, pb: 0.0, pr: 0.0 },
        Vcs { y: y4, pb: 0.0, pr: 0.0 },
    ], )
}

/// Reverts DCT and quantization operations on blocks of an image
pub fn undo_dct_on_block(compressed_vec: Vec<u32>, height: usize, width: usize) -> Array2<Vcs>{
    // Initialize an array to store Vcs data
    let temp = Vcs { y: 0.0, pb: 0.0, pr: 0.0 };
    let vcs = vec![temp; width as usize * height as usize];
    let mut vcs_image_array = Array2::from_row_major(width, height as usize, &vcs);

    // Iterate over blocks in the compressed image
    for y in (0..height as usize).step_by(2) {
        for x in (0..width as usize).step_by(2) {
            // Calculate index in the compressed vector
            let index = (y / 2 * width as usize / 2) + x / 2;

            // Unpack the compressed block
            let packed_word = compressed_vec[index];
            let (quantized_a, quantized_b, quantized_c, quantized_d, chroma_pb_index, chroma_pr_index) = unpack_block(packed_word);

            // Unwrap and convert quantized coefficients and chroma indices
            let quantized_a = quantized_a.unwrap_or(0) as u64;
            let quantized_b = quantized_b.unwrap_or(0) as i64;
            let quantized_c = quantized_c.unwrap_or(0) as i64;
            let quantized_d = quantized_d.unwrap_or(0) as i64;
            let chroma_pb_index = chroma_pb_index.unwrap_or(0) as u64;
            let chroma_pr_index = chroma_pr_index.unwrap_or(0) as u64;

            // Undo quantization
            let (dequantized_a, dequantized_b, dequantized_c, dequantized_d) = undo_quantiza_dct(quantized_a, quantized_b, quantized_c, quantized_d);
            
            // Revert DCT transformation
            let decompressed_block = undo_dct(dequantized_a, dequantized_b, dequantized_c, dequantized_d);

            // Obtain chroma values from chroma indices
            let pb = chroma_of_index(chroma_pb_index.try_into().unwrap());
            let pr = chroma_of_index(chroma_pr_index.try_into().unwrap());

            // Set reconstructed Vcs block in the Vcs image array
            for (delta_y, delta_x) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let mut pixel = decompressed_block.get(delta_x, delta_y).unwrap().clone();
                pixel.pb = pb;
                pixel.pr = pr;
                vcs_image_array.set(x + delta_x, y + delta_y, pixel);
            }
        }
    }
    vcs_image_array
}