use std::fs;

use crate::{bitboard_helpers::*, position::board::Board, constants::*, lookups::*};

impl Board {
    pub fn attackers_to_exist(&self, square: u64, occupancy: u64, color: usize) -> u64 {
        let pieces = self.get_pieces(color);
        let mut attackers: u64 = 0;
        let square_i = square.trailing_zeros() as usize;

        attackers |= self.lookup_holder.get_pawn_attacks(square_i, color) & pieces.get_pawns();
        attackers |= self.lookup_holder.get_knight_attacks(square_i) & pieces.get_knights();
        attackers |= orthogonal_attacks_to(&self.lookup_holder, square, occupancy) & pieces.get_orthogonals();
        attackers |= diagonal_attacks_to(&self.lookup_holder, square, occupancy) & pieces.get_diagonals();
        attackers |= self.lookup_holder.get_king_attacks(square_i) & pieces.get_king();

        attackers
    }
}



pub fn attacks_to(lookup_holder: &LookupHolder, piece: usize, square: u64, occupancy: u64, color: usize) -> u64 {
    match piece {
        PAWN => lookup_holder.get_pawn_attacks(square.trailing_zeros() as usize, color), 
        KNIGHT => lookup_holder.get_knight_attacks(square.trailing_zeros() as usize),
        BISHOP => diagonal_attacks_to(lookup_holder, square, occupancy),
        ROOK => orthogonal_attacks_to(lookup_holder, square, occupancy),
        QUEEN => diagonal_attacks_to(lookup_holder, square, occupancy) | orthogonal_attacks_to(lookup_holder, square, occupancy),
        KING => lookup_holder.get_king_attacks(square.trailing_zeros() as usize),
        _ => panic!("Invalid value supplied to attacks_to"),
    }
}

pub fn attacks_from(lookup_holder: &LookupHolder, piece: usize, square: u64, occupancy: u64, color: usize) -> u64 {
    match piece {
        PAWN => pawn_attacks_from(square, color),
        KNIGHT => lookup_holder.get_knight_attacks(square.trailing_zeros() as usize),
        BISHOP => diagonal_attacks_from(lookup_holder, square, occupancy),
        ROOK => orthogonal_attacks_from(lookup_holder, square, occupancy),
        QUEEN => {
            diagonal_attacks_from(lookup_holder, square, occupancy) | orthogonal_attacks_from(lookup_holder, square, occupancy)
        }
        KING => lookup_holder.get_king_attacks(square.trailing_zeros() as usize),
        _ => panic!("Invalid value supplied to attacks_from"),
    }
}

pub fn pawn_attacks_from(square: u64, color: usize) -> u64 {
    if color == WHITE {
        ((square & !FILEA) << 7) | ((square & !FILEH) << 9)
    } else {
        ((square & !FILEA) >> 9) | ((square & !FILEH) >> 7)
    }
}

pub fn orthogonal_attacks_to(lookup_holder: &LookupHolder, square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    lookup_holder.get_rook_attacks(square_i, occupancy)
}

pub fn orthogonal_attacks_from(lookup_holder: &LookupHolder, square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    lookup_holder.get_rook_attacks(square_i, occupancy)
}

pub fn diagonal_attacks_to(lookup_holder: &LookupHolder, square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    lookup_holder.get_bishop_attacks(square_i, occupancy)
}

pub fn diagonal_attacks_from(lookup_holder: &LookupHolder, square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    lookup_holder.get_bishop_attacks(square_i, occupancy)
}
