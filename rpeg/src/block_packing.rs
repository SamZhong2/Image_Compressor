use bitpack::bitpack;

/// Packs values a, b, c, d, pb, and pr into a 32-bit word
pub fn pack_block(a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64) -> u32 {
    let mut word = 0u64;

    // Pack values into a 64-bit word using bitpack functions
    word = bitpack::newu(word, 9, 23, a).unwrap();
    word = bitpack::news(word, 5, 18, b).unwrap(); 
    word = bitpack::news(word, 5, 13, c).unwrap(); 
    word = bitpack::news(word, 5, 8, d).unwrap(); 
    word = bitpack::newu(word, 4, 4, pb).unwrap(); 
    word = bitpack::newu(word, 4, 0, pr).unwrap(); 

    word as u32
}

/// Unpacks a packed 32-bit word into its constituent parts
pub fn unpack_block(packed_word: u32) -> (Option<u64>, Option<i64>, Option<i64>, Option<i64>, Option<u64>, Option<u64>) {
    
    // Unpack values from the packed word using bitpack functions
    let a = bitpack::getu(packed_word as u64, 9, 23);
    let b = bitpack::gets(packed_word as u64, 5, 18);
    let c = bitpack::gets(packed_word as u64, 5, 13);
    let d = bitpack::gets(packed_word as u64, 5, 8);
    let pb = bitpack::getu(packed_word as u64, 4, 4);
    let pr = bitpack::getu(packed_word as u64, 4, 0);
    (a, b, c, d, pb, pr)
}
