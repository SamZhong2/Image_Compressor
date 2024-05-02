use csc411_image::Rgb;
use array2::Array2;
use crate::codec;
use codec::Vcs;

/// Converts an Array2 of Vcs pixels to an Array2
pub fn vcs_to_rgb(vcs_image: &Array2<Vcs>) -> Array2<Rgb> {
    // Initialize a vector to store Rgb pixel data
    let mut rgb_vec = Vec::with_capacity(vcs_image.width() * vcs_image.height());
    
    // Iterate over the Vcs image array
    for y in 0..vcs_image.height() {
        for x in 0..vcs_image.width() {
            // Retrieve Vcs pixel at (x, y)
            let pixel = vcs_image.get(x, y).unwrap();
            
            // Convert YpbPr to Rgb
            let red = (pixel.y + 1.402 * pixel.pr).min(1.0).max(0.0) * 255.0;
            let green = (pixel.y - 0.344136 * pixel.pb - 0.714136 * pixel.pr).min(1.0).max(0.0) * 255.0;
            let blue = (pixel.y + 1.772 * pixel.pb).min(1.0).max(0.0) * 255.0;
            
            // Create Rgb pixel and push it to the vector
            rgb_vec.push(Rgb { red: red as u16, green: green as u16, blue: blue as u16 });
        }
    }
    // Convert vector to Array2 and return
    Array2::from_row_major(vcs_image.width(), vcs_image.height(), &rgb_vec)
}
