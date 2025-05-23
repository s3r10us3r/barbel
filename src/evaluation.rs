use mobility::score_mobility;
use pawn_structure::score_pawn_structure;
use phase::{get_phase_val, interp_phase};
use piece_squares::score_piece_squares;
use piece_values::{
    evaluate_pieces, simplified_king_eval_end, simplified_king_eval_mid,
};

use crate::board::{board::Board, piece_set::PieceSet};
mod board_state;
mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;
mod piece_squares;

pub fn evaluate(board: &Board) -> i32 {
    let phase = get_phase_val(board);
    let mut res = evaluate_pieces(board, phase);
    res += score_piece_squares(board, phase);
    res += score_mobility(board, board.us) - score_mobility(board, board.enemy);
    res += score_pawn_structure(board);
    res
}
