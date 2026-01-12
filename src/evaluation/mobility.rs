use crate::bitboard_helpers::pop_lsb;
use crate::constants::{BISHOP, FILES, KNIGHT, QUEEN, ROOK};
use crate::evaluation::phase::interp_phase;
use crate::moving::move_generation::{pawn_attacks_all, MoveGenerator};
use crate::position::board::Board;
use crate::position::piece_set::PieceSet;

const KNIGHT_MOBILITY: [i32; 9] = [-20, -10, -5, 0, 5, 10, 15, 20, 20]; 
const BISHOP_MOBILITY: [i32; 14] = [-20, -10, -5, 0, 5, 10, 15, 20, 25, 30, 30, 30, 30, 30];
const ROOK_MOBILITY: [i32; 15] = [-30, -20, -10, 0, 5, 10, 15, 20, 25, 30, 35, 40, 40, 40, 40];

const QUEEN_MOBILITY: [i32; 28] = [
    -20, -10, -5, 0, 5, 10, 5, 10, 10, 15, 15, 20, 20, 25, 25, 
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30
];


pub fn score_mobility(board: &Board, mg: &MoveGenerator, color: usize, enemy_color: usize, phase: i32) -> i32 {
    let pieces = board.get_pieces(color);
    let enemy_pawns = board.get_pieces(enemy_color).get_pawns();
    let pawn_attacks = pawn_attacks_all(enemy_pawns, color);

    let occ = board.get_occupancy();

    
    let us = pieces.get_all();
    let mut score = 0;
    
    score += compute_mobility(mg, pieces.get_bishops(), BISHOP, occ, us, color, &BISHOP_MOBILITY, pawn_attacks);
    score += compute_mobility(mg, pieces.get_knights(), KNIGHT, occ, us, color, &KNIGHT_MOBILITY, pawn_attacks);
    score += compute_mobility(mg, pieces.get_rooks(),   ROOK,   occ, us, color, &ROOK_MOBILITY, pawn_attacks);
    score += compute_mobility(mg, pieces.get_queens(),  QUEEN,  occ, us, color, &QUEEN_MOBILITY, pawn_attacks);
    
    let rook_bonus = open_file_rook(pieces, enemy_pawns);
    let rook_bonus = interp_phase(rook_bonus, 0, phase);

    score + rook_bonus
}

fn open_file_rook(pieces: &PieceSet, enemy_pawns: u64) -> i32 {
    let mut rooks = pieces.get_rooks();
    let pawns = pieces.get_pawns();
    let mut score = 0;
    while rooks != 0 {
        let rook_sq = pop_lsb(&mut rooks);
        let rook_file = rook_sq % 8;
        let file_bb = FILES[rook_file];
        if file_bb & pawns == 0 {
            if file_bb & enemy_pawns == 0 {
                score += OPEN_FILE_BONUS;
            } else {
                score += HALF_OPEN_FILE_BONUS;
            }
        }
    }
    score
}

#[inline]
fn compute_mobility(mg: &MoveGenerator, mut piece_mask: u64, piece: usize, occ: u64, us: u64, color: usize, table: &[i32], pawn_attacks: u64) -> i32 {
    let mut score = 0;
    
    while piece_mask != 0 {
        let sq = pop_lsb(&mut piece_mask);
        let attacks = mg.attacks_from(piece, sq, occ, color);
        let valid_moves = attacks & !us & !pawn_attacks; 
        let count = valid_moves.count_ones() as usize;
        score += table[count.min(table.len() - 1)];
    }
    score
}

const OPEN_FILE_BONUS: i32 = 60;
const HALF_OPEN_FILE_BONUS: i32 = 40;
