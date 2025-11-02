use self::piece_squares::score_piece_squares;
use self::piece_values::evaluate_pieces;

use crate::constants::WHITE;
use crate::evaluation::pawn_structure::{PawnEvalHashTable};
use crate::position::board::Board;
mod board_state;
pub mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;
mod piece_squares;
mod king_safety;
mod preliminary;

pub struct Evaluator {
    pawn_hash: PawnEvalHashTable,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { pawn_hash: PawnEvalHashTable::new() }
    }

    pub fn evaluate(&mut self, board: &Board) -> i32 {
        let pre_eval_result = self.run_pre_eval(board);
        let pieces = evaluate_pieces(board, &pre_eval_result);
        let piece_squares = score_piece_squares(board, &pre_eval_result);
        let pawn_score = self.score_pawns(board, &pre_eval_result);
        let score = pieces + piece_squares + pawn_score;
        if board.us == WHITE {
            score
        } else {
            -score
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

