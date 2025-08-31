use self::phase::get_phase_val;
use self::piece_squares::score_piece_squares;
use self::piece_values::evaluate_pieces;

use crate::moving::move_generation::MoveGenerator;
use crate::position::board::Board;
mod board_state;
mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;
mod piece_squares;

pub fn evaluate(board: &Board, mg: &MoveGenerator) -> i32 {
    let phase = get_phase_val(board);
    let pieces = evaluate_pieces(board, phase);
    let piece_squares = score_piece_squares(board);
    pieces + piece_squares 
}

