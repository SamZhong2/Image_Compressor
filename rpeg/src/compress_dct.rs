use crate::block_packing::pack_block;
use array2::Array2;

use crate::codec;
use codec::Vcs;


/// Applies Discrete Cosine Transform on a 2x2 block of vcs values.
pub fn transform_to_dct(block: &Array2<Vcs>) -> (f32, f32, f32, f32) {
    // Extract luminance values of the block
    let y1 = block.get(0, 0).unwrap().y;
    let y2 = block.get(1, 0).unwrap().y;
    let y3 = block.get(0, 1).unwrap().y;
    let y4 = block.get(1, 1).unwrap().y;

    // Calculate DCT coefficients
    let a = (y1 + y2 + y3 + y4) / 4.0;
    let b = (y4 + y3 - y2 - y1) / 4.0;
    let c = (y4 - y3 + y2 - y1) / 4.0;
    let d = (y4 - y3 - y2 + y1) / 4.0;
    

    (a, b, c, d)
    
}

/// Quantizes a single DCT coefficient
fn quantize_coeff(val: f32) -> i64 {
    if val.abs() > 0.3 {
        if val > 0.0 { 15 } else { -15 }
    } else {
        (val * 50.0).round() as i64
    }
}

/// Quantizes all DCT coefficients
pub fn quantize_dct(a: f32, b: f32, c: f32, d: f32) -> (u64, i64, i64, i64) {
    let a_quan = (a * 511.0).round() as u64;
    let b_quan = quantize_coeff(b);
    let c_quan = quantize_coeff(c);
    let d_quan = quantize_coeff(d);

    (a_quan, b_quan, c_quan, d_quan)
}

/// Computes the average of chroma values for a block of vcs pixels
pub fn average_of_vcs(block: &Array2<Vcs>) -> (u64, u64) {
    let mut pbs = Vec::new();
    let mut prs = Vec::new();

    for y in 0..block.height() {
        for x in 0..block.width() {
            let pixel = block.get(x, y).unwrap();
            pbs.push(pixel.pb);
            prs.push(pixel.pr);
        }
    }

    let pb = csc411_arith::index_of_chroma(pbs.iter().sum::<f32>() / pbs.len() as f32);
    let pr = csc411_arith::index_of_chroma(prs.iter().sum::<f32>() / prs.len() as f32);
    (pb as u64, pr as u64)
}

/// Helper to calculates DCT coefficients and quantizes them for a block
fn calculate_dct_and_quantize(block: &Array2<Vcs>) -> (u64, i64, i64, i64) {
    let (dct_a, dct_b, dct_c, dct_d) = transform_to_dct(&block);
    quantize_dct(dct_a, dct_b, dct_c, dct_d)
}

/// Helper to extracts a 2x2 block from the given image
fn get_block(img: &Array2<Vcs>, x: usize, y: usize) -> Array2<Vcs> {
    let block_data = Array2::from_row_major(2, 2, &vec![
        img.get(x, y).unwrap().clone(),
        img.get(x + 1, y).unwrap().clone(),
        img.get(x, y + 1).unwrap().clone(),
        img.get(x + 1, y + 1).unwrap().clone()]);

        block_data      
}

/// Performs DCT and quantization on blocks of an image
pub fn dct_on_block(img: Array2<Vcs>, height: usize, width: usize) -> Vec<u32>{
    let mut temp_vec:Vec<u32> = Vec::new();

    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            
            let block = get_block(&img, x, y);

            let (a_q, b_q, c_q, d_q) = calculate_dct_and_quantize(&block);

            let (pb_chroma, pr_chroma) = average_of_vcs(&block);

            let packed_word = pack_block(a_q, b_q, c_q, d_q, pb_chroma, pr_chroma);

            
            temp_vec.push(packed_word);
        }

        
    }

    temp_vec
}