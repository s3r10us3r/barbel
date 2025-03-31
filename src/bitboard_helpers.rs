#[inline]
pub fn pop_lsb(bitboard: &mut u64) -> u32 {
    let result = bitboard.trailing_zeros();
    *bitboard &= *bitboard - 1;
    result
}

#[inline]
pub fn isolate_msb(bitboard: u64) -> u64 {
    1 << (63 - bitboard.leading_zeros())
}

#[inline]
pub fn isolate_lsb(bitboard: u64) -> u64 {
    if bitboard == 0 {
        0
    } else {
        bitboard & !(bitboard - 1)
    }
}

#[inline]
pub fn has_more_than_one(bitboard: u64) -> bool {
    bitboard & (bitboard - 1) != 0
}

#[inline]
pub fn get_lsb(bitboard: &u64) -> usize {
    bitboard.trailing_zeros() as usize
}

#[inline]
pub fn get_msb(bitboard: &u64) -> usize {
    (63 - bitboard.leading_zeros()) as usize
}
