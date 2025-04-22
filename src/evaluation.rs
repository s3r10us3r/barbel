use piece_values::{
    evaluate_pieces, simplified_eval, simplified_king_eval_end, simplified_king_eval_mid,
};

use crate::board::{board::Board, piece_set::PieceSet};
mod board_state;
mod piece_values;

pub fn evaluate(board: &Board) -> i32 {
    let us = board.get_pieces(board.us);
    let enemy = board.get_pieces(board.enemy);
    let mut res = evaluate_pieces(us) - evaluate_pieces(enemy);
    res += simplified_eval(us) - simplified_eval(enemy);
    if is_end_game(us, enemy) {
        res += simplified_king_eval_end(us) - simplified_king_eval_end(enemy);
    } else {
        res += simplified_king_eval_mid(us) - simplified_king_eval_end(enemy);
    }
    res
}

fn is_end_game(us: &PieceSet, enemy: &PieceSet) -> bool {
    if us.get_queens() == 0 && enemy.get_queens() == 0 {
        true
    } else {
        minor_piece_count(us) == 1 && minor_piece_count(enemy) == 1
    }
}

fn minor_piece_count(ps: &PieceSet) -> u32 {
    (ps.get_bishops() | ps.get_rooks() | ps.get_knights()).count_ones()
}
