use crate::lookups::{magics::*, simple_lookups::{BB_BETWEEN, KING_LOOKUP, KNIGHT_LOOKUP, PAWN_ATTACKS_TO}};

mod magics;
mod simple_lookups;
mod pawn_move_gen;
mod legality;

pub const N8: usize = 0;
pub const NE8: usize = 1;
pub const E8: usize = 2;
pub const SE8: usize = 3;
pub const S8: usize = 4;
pub const SW8: usize = 5;
pub const W8: usize = 6;
pub const NW8: usize = 7;



pub struct LookupHolder {
    rook_lookup: Vec<Vec<u64>>,
    bishop_lookup: Vec<Vec<u64>>, }

impl LookupHolder {
    pub fn new() -> Self {
        LookupHolder { rook_lookup: compute_rook_lookup(), bishop_lookup: compute_bishop_lookup() }
    }

    pub fn get_knight_attacks(&self, sq: usize) -> u64 {
        KNIGHT_LOOKUP[sq]
    }

    pub fn get_king_attacks(&self, sq: usize) -> u64 {
        KING_LOOKUP[sq]
    }

    pub fn get_pawn_attacks(&self, sq: usize, color: usize) -> u64 {
        PAWN_ATTACKS_TO[color][sq]
    }

    pub fn get_rook_attacks(&self, sq: usize, occ: u64) -> u64 {
        let bb = occ & ROOK_RELEVANCY_MASKS[sq];
        let idx = index_magic(bb, ROOK_MAGICS[sq], ROOK_SHIFTS[sq]);
        self.rook_lookup[sq][idx]
    }

    pub fn get_bishop_attacks(&self, sq: usize, occ: u64) -> u64 {
        let bb = occ & BISHOP_RELEVANCY_MASKS[sq];
        let idx = index_magic(bb, BISHOP_MAGICS[sq], BISHOP_SHIFTS[sq]);
        self.bishop_lookup[sq][idx]
    }

    pub fn get_bb_between(&self, sq1: usize, sq2: usize) -> u64 {
        BB_BETWEEN[sq1][sq2]
    }
}
