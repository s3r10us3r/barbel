use crate::{board::{board::Board, piece_set::PieceSet}, constants::{ROOK, WHITE}};

pub fn get_phase_weight(board: &Board) -> f32 {
    let ps1 = board.get_pieces(board.us);
    let ps2 = board.get_pieces(board.us);
    (count_pieceset_game_phase(ps1) + count_pieceset_game_phase(ps2)) / INIT_PHASE_VALUE
}

fn count_pieceset_game_phase(ps: &PieceSet) -> f32 {
    let mut sum = (ps.get_pawns().count_ones() as f32) * PAWN_PHASE_VALUE;
    sum += (ps.get_knights().count_ones() as f32) * KNIGHT_PHASE_VALUE;
    sum += (ps.get_bishops().count_ones() as f32)  * BISHOP_PHASE_VALUE;
    sum += (ps.get_rooks().count_ones() as f32) * ROOK_PHASE_VALUE;
    sum += (ps.get_queens().count_ones() as f32) * QUEEN_PHASE_VALUE;
    sum
}

const PAWN_PHASE_VALUE: f32 = 0.;
const KNIGHT_PHASE_VALUE: f32 = 1.;
const BISHOP_PHASE_VALUE: f32 = 1.;
const ROOK_PHASE_VALUE: f32 = 2.;
const QUEEN_PHASE_VALUE: f32 = 4.;

const INIT_PHASE_VALUE: f32 = PAWN_PHASE_VALUE * 16.
    + KNIGHT_PHASE_VALUE * 4.
    + BISHOP_PHASE_VALUE * 4.
    + ROOK_PHASE_VALUE * 4.
    + QUEEN_PHASE_VALUE * 2.;
