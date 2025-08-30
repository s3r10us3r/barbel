use crate::{bitboard_helpers::pop_lsb, constants::{BISHOP, KNIGHT, QUEEN, ROOK}, moving::move_generation::MoveGenerator, position::board::Board};

pub fn score_mobility(board: &Board, mg: &MoveGenerator, color: usize) -> i32 {
    let pieces = board.get_pieces(color);
    let occ = board.get_occupancy();
    let mut score = 0;
    score += compute_mobility(mg, pieces.get_bishops(), BISHOP, occ, color, BISHOP_BONUS);
    score += compute_mobility(mg, pieces.get_knights(), KNIGHT, occ, color, KNIGHT_BONUS);
    score += compute_mobility(mg, pieces.get_rooks(),  ROOK, occ, color, ROOK_BONUS);
    score += compute_mobility(mg, pieces.get_queens(), QUEEN, occ, color, QUEEN_BONUS);
    score
}

#[inline]
fn compute_mobility(mg: &MoveGenerator, mut piece_mask: u64, piece: usize, occ: u64, color: usize, bonus: i32) -> i32 {
    let mut cnt = 0;
    while piece_mask != 0 {
        let sq = pop_lsb(&mut piece_mask);
        cnt += mg.attacks_from(piece, 1 << sq, occ, color).count_ones();
    }
    (cnt as i32) * bonus
}

const BISHOP_BONUS: i32 = 5;
const KNIGHT_BONUS: i32 = 5;
const ROOK_BONUS: i32 = 2;
const QUEEN_BONUS: i32 = 1;

