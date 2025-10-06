use crate::{
    constants::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK}, evaluation::preliminary::PreEvalResult, position::{board::Board, piece_set::PieceSet}
};

use super::phase::interp_phase;

struct PieceValues {
    values: [i32; 5],
}

//NONE, PAWN, KNIGHT, BISHOP, ROOK, QUEEN
const MIDGAME_PIECE_VALUES: PieceValues = PieceValues{values: [100,300,350,455,900]};
const ENDGAME_PIECE_VALUES: PieceValues = PieceValues{values: [150,300,350,550,1000]};

pub fn evaluate_pieces(board: &Board, pre_eval_result: &PreEvalResult) -> i32 {
    let midgame_sum = evaluate_pieces_w_vals(board.get_pieces(board.us), &MIDGAME_PIECE_VALUES)
        - evaluate_pieces_w_vals(board.get_pieces(board.enemy), &MIDGAME_PIECE_VALUES);
    let endgame_sum = evaluate_pieces_w_vals(board.get_pieces(board.us), &ENDGAME_PIECE_VALUES)
        - evaluate_pieces_w_vals(board.get_pieces(board.enemy), &ENDGAME_PIECE_VALUES);
    interp_phase(midgame_sum, endgame_sum, pre_eval_result.phase)
}

fn evaluate_pieces_w_vals(piece_set: &PieceSet, piece_values: &PieceValues) -> i32 {
    let mut res = piece_set.get_pawns().count_ones() as i32 * piece_values.values[PAWN];
    res += piece_set.get_knights().count_ones() as i32 * piece_values.values[KNIGHT];
    res += piece_set.get_bishops().count_ones() as i32 * piece_values.values[BISHOP];
    res += piece_set.get_rooks().count_ones() as i32 * piece_values.values[ROOK];
    res += piece_set.get_queens().count_ones() as i32 * piece_values.values[QUEEN];
    res
}
