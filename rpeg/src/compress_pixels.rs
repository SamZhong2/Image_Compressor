use array2::Array2;
use csc411_image::Rgb;

use crate::codec;
use codec::{RgbFloat, Vcs};


/// Adjusts the width of an image to ensure it is even
pub fn even_width(img_array: Array2<Rgb>) -> usize{
    if img_array.width() % 2 != 0 { 
        img_array.width() - 1 
    } 
    else { 
        img_array.width()
    }
}

/// Adjusts the height of an image to ensure it is even
pub fn even_height(img_array: Array2<Rgb>) -> usize{
    if img_array.height() % 2 != 0 { 
        img_array.height() - 1 
    } 
    else { 
        img_array.height()
    }
}

/// Trims an image to have even width and height
pub fn even_array2(image: Array2<Rgb>, width: usize, height: usize) -> Array2<Rgb> {
    let mut temp_data = Vec::with_capacity(width * height);
   
    for y in 0..height {
        for x in 0..width {
            if let Some(pixel) = image.get(x, y) {
                temp_data.push(pixel.clone());
            }
        }
    }

    Array2::from_row_major(width, height, &temp_data)
}

/// Converts an image from Rgb format to RgbFloat format
pub fn convert_to_floats(image: Array2<Rgb>) -> Array2<RgbFloat> {
    let mut float_data = Vec::with_capacity(image.width() * image.height());
    
    for ( _row, _col, val) in image.iter_row_major(){
        let r = val.red as f32 / 255.0;
        let g = val.green as f32 / 255.0;
        let b = val.blue as f32 / 255.0;
        float_data.push(RgbFloat { red: r, green: g, blue: b });
    }

    Array2::from_row_major(image.width(), image.height(), &float_data)
}

/// Converts an image from RgbFloat format to vcs format
pub fn convert_to_vcs(image: Array2<RgbFloat>) -> Array2<Vcs>{
    let mut vcs_data = Vec::with_capacity(image.width() * image.height());

    for ( _row, _col, val) in image.iter_row_major(){
        let y = 0.299 * val.red + 0.587 * val.green + 0.114 * val.blue;
        let pb = -0.168736 * val.red - 0.331264 * val.green + 0.5 * val.blue;
        let pr = 0.5 * val.red - 0.418688 * val.green - 0.081312 * val.blue;
        vcs_data.push(Vcs { y, pb, pr });
    }

    Array2::from_row_major(image.width(), image.height(), &vcs_data)

}