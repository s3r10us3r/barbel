use self::piece_squares::score_piece_squares;
use self::piece_values::evaluate_pieces;

use crate::constants::{BLACK, WHITE};
use crate::evaluation::mobility::score_mobility;
use crate::evaluation::pawn_structure::{PawnEvalHashTable};
use crate::moving::move_generation::MoveGenerator;
use crate::position::board::Board;
mod board_state;
pub mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;
mod piece_squares;
mod king_safety;
mod preliminary;
mod greed;

pub struct Evaluator {
    pawn_hash: PawnEvalHashTable,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { pawn_hash: PawnEvalHashTable::new() }
    }

    pub fn evaluate(&mut self, board: &Board, mg: &MoveGenerator) -> i32 {
        let pre_eval_result = self.run_pre_eval(board);
        let pieces = evaluate_pieces(board, &pre_eval_result);
        let piece_squares = score_piece_squares(board, &pre_eval_result);
        let pawn_score = self.score_pawns(board, &pre_eval_result);
        let king_safety = self.evaluate_king_safety(board, mg);
        let mobility_score = score_mobility(board, mg, WHITE, BLACK, pre_eval_result.phase) - score_mobility(board, mg, BLACK, WHITE, pre_eval_result.phase);
        let greed_score = self.score_queen_greed(board, pre_eval_result.phase);
        let score = pieces + piece_squares + pawn_score + king_safety + mobility_score + greed_score;
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

