use crate::constants::*;

#[inline]
pub fn pop_lsb(bitboard: &mut u64) -> usize {
    let result = bitboard.trailing_zeros();
    *bitboard &= *bitboard - 1;
    result as usize
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

#[inline]
pub fn reverse(mut bb: u64) -> u64 {
    //swaps 8 bit groups
    bb = ((bb >> 8) & 0x00FF00FF00FF00FF) | ((bb & 0x00FF00FF00FF00FF)  << 8);
    //swaps 16 bit groups (two ranks)
    bb = ((bb >> 16) & 0x0000FFFF0000FFFF) | ((bb & 0x0000FFFF0000FFFF) << 16);
    //swaps 32 bit group (four ranks)
    bb.rotate_left(32)
}

#[inline]
pub fn get_file(bb: u64) -> usize {
    (bb.count_ones() % 8) as usize
}

#[inline]
pub fn get_rank(bb: u64) -> usize {
    (bb.count_ones() / 8) as usize
}

// DIRECTIONS
#[inline]
pub fn move_n(sq: u64) -> u64 {
    (sq & !RANK8) << 8
}

#[inline]
pub fn move_ne(sq: u64) -> u64 {
    (sq & !(RANK8 | FILEH)) << 9
}

#[inline]
pub fn move_e(sq: u64) -> u64 {
    (sq & !FILEH) << 1
}

#[inline]
pub fn move_se(sq: u64) -> u64 {
    (sq & !(RANK1 | FILEH)) >> 7
}

#[inline]
pub fn move_s(sq: u64) -> u64 {
    (sq & !RANK1) >> 8
}

#[inline]
pub fn move_sw(sq: u64) -> u64 {
    (sq & !(RANK1 | FILEA)) >> 9
}

#[inline]
pub fn move_w(sq: u64) -> u64 {
    (sq & !FILEA) >> 1
}

#[inline]
pub fn move_nw(sq: u64) -> u64 {
    (sq & !(RANK8 | FILEA)) << 7
}

#[inline]
pub fn flip_color(color: usize) -> usize {
    color ^ 1
}

pub const MOVE_FUNCS: [fn(u64) -> u64; 8] = [move_n, move_ne, move_e, move_se, move_s, move_sw, move_w, move_nw];
