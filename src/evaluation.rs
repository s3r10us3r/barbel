use self::phase::get_phase_val;
use self::piece_squares::score_piece_squares;
use self::piece_values::evaluate_pieces;

use crate::evaluation::pawn_structure::{score_pawns, PawnEvalHashTable};
use crate::moving::move_generation::MoveGenerator;
use crate::position::board::Board;
mod board_state;
mod piece_values;
mod pawn_structure;
mod mobility;
mod phase;
mod piece_squares;

pub struct Evaluator {
    pawn_hash: PawnEvalHashTable,
    phase: i32,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { pawn_hash: PawnEvalHashTable::new(), phase: 0 }
    }

    pub fn evaluate(&mut self, board: &Board, mg: &MoveGenerator) -> i32 {
        self.phase = get_phase_val(board);
        let pieces = evaluate_pieces(board, self.phase);
        let piece_squares = score_piece_squares(board);
        let (us, enemy) = board.get_piecesets();
        let pawn_score = if let Some(entry) = self.pawn_hash.probe(us.get_pawns(), enemy.get_pawns()) {
            entry.score
        } else {
            let score = score_pawns(us, enemy, self.phase) - score_pawns(enemy, us, self.phase);
            self.pawn_hash.store(us.get_pawns(), enemy.get_pawns(), score);
            score
        };
        pieces + piece_squares + pawn_score
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

