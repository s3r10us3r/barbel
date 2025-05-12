use mobility::score_mobility;
use pawn_structure::score_pawn_structure;
use phase::get_phase_weight;
use piece_values::{
    evaluate_pieces, simplified_king_eval_end, simplified_king_eval_mid,
};

use crate::board::{board::Board, piece_set::PieceSet};
mod board_state;
mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;

pub fn evaluate(board: &Board) -> i32 {
    let phase = get_phase_weight(board);
    let us = board.get_pieces(board.us);
    let enemy = board.get_pieces(board.enemy);
    let mut res = evaluate_pieces(board, phase);
    let end_eval = ((1. - phase) * (simplified_king_eval_end(us) - simplified_king_eval_end(enemy)) as f32) as i32;
    let mid_eval = (phase * (simplified_king_eval_mid(us) - simplified_king_eval_mid(enemy)) as f32) as i32;
    res += end_eval + mid_eval;
    res += score_mobility(board, board.us) - score_mobility(board, board.enemy);
    res += score_pawn_structure(board);
    res
}
