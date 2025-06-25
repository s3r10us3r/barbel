use crate::board::{board::Board, piece_set::PieceSet};

pub fn interp_phase(mg_val: i32, eg_val: i32, phase: i32) -> i32 {
    if phase == 0 {
        eg_val
    } else if phase > INIT_PHASE_VALUE as i32 {
        mg_val
    }
    else {
        (phase * mg_val + (INIT_PHASE_VALUE as i32 - phase) * eg_val) / INIT_PHASE_VALUE as i32
    }
}

pub fn get_phase_val(board: &Board) -> i32 {
    let ps1 = board.get_pieces(board.us);
    let ps2 = board.get_pieces(board.enemy);
    (count_pieceset_game_phase(ps1) + count_pieceset_game_phase(ps2)) as i32
}

fn count_pieceset_game_phase(ps: &PieceSet) -> u32 {
    let mut sum = ps.get_pawns().count_ones() * PAWN_PHASE_VALUE;
    sum += ps.get_knights().count_ones() * KNIGHT_PHASE_VALUE;
    sum += ps.get_bishops().count_ones() * BISHOP_PHASE_VALUE;
    sum += ps.get_rooks().count_ones() * ROOK_PHASE_VALUE;
    sum += ps.get_queens().count_ones() * QUEEN_PHASE_VALUE;
    sum
}

const PAWN_PHASE_VALUE: u32 = 0;
const KNIGHT_PHASE_VALUE: u32 = 1;
const BISHOP_PHASE_VALUE: u32 = 1;
const ROOK_PHASE_VALUE: u32 = 2;
const QUEEN_PHASE_VALUE: u32 = 4;

const INIT_PHASE_VALUE: u32 = PAWN_PHASE_VALUE * 16
    + KNIGHT_PHASE_VALUE * 4
    + BISHOP_PHASE_VALUE * 4
    + ROOK_PHASE_VALUE * 4
    + QUEEN_PHASE_VALUE * 2;
