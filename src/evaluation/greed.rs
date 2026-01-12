use crate::{constants::{BLACK, WHITE}, evaluation::{phase::interp_phase, Evaluator}, position::board::Board};

const WHITE_POISON_SQUARES: u64 = (1 << 49) | (1 << 54); // b7, g7
const BLACK_POISON_SQUARES: u64 = (1 << 9) | (1 << 14);  // b2, g2

const EARLY_QUEEN_PENALTY: i32 = 60; 

impl Evaluator {
    pub fn score_queen_greed(&self, board: &Board, phase: i32) -> i32 {
        let white_queens = board.get_pieces(WHITE).get_queens();
        let black_queens = board.get_pieces(BLACK).get_queens();
        
        let mut score = 0;

        if (white_queens & WHITE_POISON_SQUARES) != 0 {
            score -= EARLY_QUEEN_PENALTY;
        }

        if (black_queens & BLACK_POISON_SQUARES) != 0 {
            score += EARLY_QUEEN_PENALTY;
        }

        
        let white_deep = white_queens & 0xFF00000000000000; 
        if white_deep != 0 {
             score -= 20; // Lekka kara za kozaczenie
        }

        let black_deep = black_queens & 0x00000000000000FF;
        if black_deep != 0 {
             score += 20;
        }

        interp_phase(score, 0, phase)
    }
}
