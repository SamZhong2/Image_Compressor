use crate::compress_pixels;
use crate::uncompress_dct;
use crate::uncompress_pixels;
use crate::compress_dct;

use csc411_image::{Read, Write, RgbImage};
use array2::Array2;
use csc411_rpegio::{input_rpeg_data,output_rpeg_data};
use compress_dct::dct_on_block;
use compress_pixels::{even_height, even_width, even_array2, convert_to_floats, convert_to_vcs};
use uncompress_dct::undo_dct_on_block;
use uncompress_pixels::vcs_to_rgb;



// Struct representing RGB pixel values as floats
#[derive(Debug, Clone, PartialEq)]
pub struct RgbFloat {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

// Struct representing Vcs values
#[derive(Debug, Clone, PartialEq)]
pub struct Vcs {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}



// Function to compress an image
pub fn compress(filename: Option<&str>) {
    // Read the RGB image from file
    let img = RgbImage::read(filename.as_deref()).unwrap();

    // Convert image pixels to a 2D array
    let img_array = Array2::from_row_major( img.width as usize, img.height as usize, &img.pixels);
    // For every pixel seen in the image, they
    // should be stored in the row major in the
    // correct spot in the array2 class

    // Ensure even width and height for blocking
    let width:usize = even_width(img_array.clone());
    let height:usize = even_height(img_array.clone());

    // Change array to even dimensions
    let even_img_array = even_array2(img_array.clone(), width, height);
    // For every image, the dimensions
    // of width and height should be
    // an even number

    // Convert pixel values to floats
    let float_img_array = convert_to_floats(even_img_array);
    // For every pixel seen even image,
    // they should be represented by RgbFloats

    // Convert RGB values to YPbPr color space values
    let vcs_img_array = convert_to_vcs(float_img_array);
    // For every Rgb value seen in 
    // the even image, they should be 
    // represented by Vcs

    // Compress the image using Discrete Cosine Transform
    let compressed_vec = dct_on_block(vcs_img_array, height, width);
    // For every Vcs value seen in 
    // the even image, they should be
    // compressed and stored in the vec
    
    // Convert compressed data to bytes
    let compressed_img: Vec<[u8; 4]> = compressed_vec.clone().into_iter().map(u32::to_be_bytes).collect();
    // For every data in compressed 
    // vec, they should be represented 
    // in bytes

    // Output the compressed data
    output_rpeg_data(&compressed_img, width, height).unwrap();
    
}

// Function to decompress an image
pub fn decompress(filename: Option<&str>) {
    // Read compressed image data
    let (compressed_img, width, height) = input_rpeg_data(filename).unwrap();

    // Convert bytes back to compressed data
    let compressed_vec: Vec<u32> = compressed_img.into_iter().map(u32::from_be_bytes).collect();
    // For every byte seen in the rpeg
    // they should be stored in the vec
    // as compressed data

    // Decompress the image using inverse Discrete Cosine Transform
    let vcs_image = undo_dct_on_block(compressed_vec, height, width);
    // For every compressed data seen in 
    // the rpeg file, they should be in 
    // Vcs format

    // Convert YPbPr color space values back to RGB
    let decompressed_rgb_array = vcs_to_rgb(&vcs_image);
    // For every Vcs value seen in 
    // the rpeg file, they should be inn
    // Rgb format

    // Convert the 2D array back to a 1D vector of RGB pixels
    let mut decompressed_rgb_pixels = Vec::new();

    for y in 0..decompressed_rgb_array.height() {
        for x in 0..decompressed_rgb_array.width() {
            decompressed_rgb_pixels.push(decompressed_rgb_array.get(x, y).unwrap().clone());
        }
    }
    
    // Create an RGB image from the decompressed pixels
    let decompressed_rgb_image = RgbImage {
        pixels: decompressed_rgb_pixels,
        width: width as u32,
        height: height as u32,
        denominator: 255,  
    };
    // For every Rgb pixel seen,
    // they should be store in the
    // image file

    // Write standard output
    if let Err(e) = RgbImage::write(&decompressed_rgb_image, None) {
        eprintln!("Failed to write image: {}", e);
    }

}