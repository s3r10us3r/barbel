use self::piece_squares::score_piece_squares;
use self::piece_values::evaluate_pieces;

use crate::evaluation::pawn_structure::{PawnEvalHashTable};
use crate::position::board::Board;
mod board_state;
mod piece_values;
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
        let piece_squares = score_piece_squares(board);
        let (us, enemy) = board.get_piecesets();
  //      let pawn_score_entry = self.score_pawns(us, enemy, &pre_eval_result);
  //      let king_safety = self.evaluate_king_safety(board, board.us, &pre_eval_result) - self.evaluate_king_safety(board, board.enemy, &pre_eval_result);
        pieces + piece_squares //+ pawn_score_entry.score + king_safety
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

