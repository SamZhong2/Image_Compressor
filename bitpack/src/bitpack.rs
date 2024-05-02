// use std::convert::TryInto;


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    // Check if the signed value fits within the specified width
    n >= (-(1 << (width - 1))) && n <= ((1 << (width - 1)) - 1)

}



// /// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
// /// 
// /// # Arguments:
// /// * `n`: An usigned integer value
// /// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    // Check if the unsigned value fits within the specified width
    n <= (1 << width) - 1

}

// /// Retrieve a signed value from `word`, represented by `width` bits
// /// beginning at least-significant bit `lsb`.
// /// 
// /// # Arguments:
// /// * `word`: An unsigned word
// /// * `width`: the width of a bit field
// /// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {

    let val = (word >> lsb) & ((1 << width) - 1);
    
    if (val & (1 << (width - 1))) != 0 {
        // Perform sign extension if the value is negative
        let sign_extension = !0 << width;
        Some((val | sign_extension) as i64)
    } else {
        Some(val as i64)
    }

}


// /// Retrieve an unsigned value from `word`, represented by `width` bits
// /// beginning at least-significant bit `lsb`.
// /// 
// /// # Arguments:
// /// * `word`: An unsigned word
// /// * `width`: the width of a bit field
// /// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {

    let val = (word >> lsb) & ((1 << width) - 1);
    Some(val)

}


// /// Return a modified version of the unsigned `word`,
// /// which has been updated so that the `width` bits beginning at
// /// least-significant bit `lsb` now contain the unsigned `value`.
// /// Returns an `Option` which will be None iff the value does not fit
// /// in `width` unsigned bits.
// /// 
// /// # Arguments:
// /// * `word`: An unsigned word
// /// * `width`: the width of a bit field
// /// * `lsb`: the least-significant bit of the bit field
// /// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    
    if !fitsu(value, width) {
        return None;
    }

    let clear_mask = !(((1 << width) - 1) << lsb);
    let modified_word = (word & clear_mask) | (value << lsb);

    Some(modified_word)
   
}



// /// Return a modified version of the unsigned `word`,
// /// which has been updated so that the `width` bits beginning at
// /// least-significant bit `lsb` now contain the signed `value`.
// /// Returns an `Option` which will be None iff the value does not fit
// /// in `width` signed bits.
// /// 
// /// # Arguments:
// /// * `word`: An unsigned word
// /// * `width`: the width of a bit field
// /// * `lsb`: the least-significant bit of the bit field
// /// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    
    if !fitss(value, width) {
        return None;
    }

    let mask = (1_u64 << width) -1;
    Some(word | ((value as u64 & mask) << lsb)) 
    
}





