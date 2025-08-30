use self::mobility::score_mobility;
use self::pawn_structure::score_pawn_structure;
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

pub fn evaluate_debug(board: &Board, mg: &MoveGenerator) {
    let phase = get_phase_val(board);
    let pieces = evaluate_pieces(board, phase);
    let piece_squares = score_piece_squares(board);
    let mobility = score_mobility(board, mg, board.us) - score_mobility(board, mg,board.enemy);
    let pawn_structure = score_pawn_structure(board);
    println!("info phase: {phase}, pieces: {pieces}, piece_squares: {piece_squares}, mobility: {mobility}, pawn_structure: {pawn_structure}");
}

