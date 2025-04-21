use piece_values::{evaluate_pieces, simplified_eval};

use crate::board::board::Board;
mod board_state;
mod piece_values;

pub fn evaluate(board: &Board) -> i32 {
    let us_piece_set = board.get_pieces(board.us);
    let enemy_piece_set = board.get_pieces(board.enemy);
    let mut res = evaluate_pieces(&us_piece_set) - evaluate_pieces(&enemy_piece_set);
    res += simplified_eval(&us_piece_set) - simplified_eval(&enemy_piece_set);
    res
}
