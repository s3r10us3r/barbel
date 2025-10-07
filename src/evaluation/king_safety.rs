use std::ops::Range;

use crate::{bitboard_helpers::{get_file, get_lsb, get_rank, isolate_lsb}, constants::{BISHOP, FILES, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE}, evaluation::{phase::interp_phase, preliminary::PreEvalResult, Evaluator}, fen_parsing::parse_to_fen, moving::move_generation::MoveGenerator, position::{board::Board, piece_set::PieceSet}};

impl Evaluator {
    pub fn evaluate_king_safety(&self, board: &Board, color: usize, pre_eval_result: &PreEvalResult) -> i32 {
        let mut score = 0;
        let us = &board.players[color];
        if !(board.get_state().can_castle_kingside(color) || board.get_state().can_castle_queenside(color)) {
            score += self.score_pawn_shield(us.get_king(), us.get_pawns(), color);
        }
        let pawn_shield_score = interp_phase(score, 0, pre_eval_result.phase);
        let king_safety_score = self.score_king_zone_attacks(get_lsb(&us.get_king()), &board.mg, pre_eval_result, color);
        pawn_shield_score + king_safety_score
    }

    /*
    * this function returns a penalty (already negated) for a pawn shield - pawns guarding the king
    * after castle
    */
    fn score_pawn_shield(&self, king: u64, pawns: u64, color: usize) -> i32 {
        let mut penalty = 0;
        let file = get_file(king);
        let king_sq = king.trailing_zeros() as i32;
        let dir = if color == WHITE { 1i32 } else { -1i32 };

        //left
        if file > 0 {
            penalty += self.king_shield_file_pen(king_sq - 1, dir, pawns);
        }
        //in front 
        penalty += self.king_shield_file_pen(king_sq, dir, pawns);
        //right 
        if file < 7 {
            penalty += self.king_shield_file_pen(king_sq + 1, dir, pawns);
        }
        penalty
    }

    // BUG: shift overflow
    fn king_shield_file_pen(&self, mut sq: i32, dir: i32, pawns: u64) -> i32 {
        //this is the only place this can overflow since there are no pawns on the last rank
        sq += 8 * dir;
        if !(0..64).contains(&sq) {
            return 0;
        }
        let sq_bb = 1u64 << sq;

        if sq_bb & pawns != 0 {
            return 0;
        }
        sq += 8 * dir;

        let sq_bb = 1u64 << sq;
        if sq_bb & pawns != 0 {
           return ONE_SQUARE_PENALTY; 
        }

        sq += 8 * dir;

        let sq_bb = 1u64 << sq;
        if sq_bb & pawns != 0 {
            TWO_SQUARE_PENALTY
        } else {
            OPEN_FILE_PENALTY
        }
    }

    // king zone is defined by a zone the king can move to + 2 rows in the enemy direction
    // for every enemy piece we take its index value and multiply it by the number of squares
    // attacked in the king zone, by said piece
    fn score_king_zone_attacks(&self, king_sq: usize, mg: &MoveGenerator, pre_eval_result: &PreEvalResult, color: usize) -> i32 {
        let king_zone = find_king_zone(king_sq, mg, color);
        let am = &pre_eval_result.attack_maps[color ^ 1];

        let mut idx = 0;
        let pieces = [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING];
        for piece in pieces {
            idx += (king_zone & am.attack_maps[piece]).count_ones() as usize * PIECE_ATTACK_ID_VALUES[piece];
        }

        -SAFETY_TABLE[idx]
    }
}

fn find_king_zone(king_sq: usize, mg: &MoveGenerator, color: usize) -> u64 {
    let king_attack = mg.get_king_attacks(king_sq);
    if color == WHITE {
        king_attack | (king_attack << 8) | (king_attack << 16)
    } else {
        king_attack | (king_attack >> 8) | (king_attack >> 16)
    }
}

const ONE_SQUARE_PENALTY: i32 = -10;
const TWO_SQUARE_PENALTY: i32 = -25;
const OPEN_FILE_PENALTY: i32 = -50;

const PIECE_ATTACK_ID_VALUES: [usize; 6]  = [1,2,2,3,5,3];


const SAFETY_TABLE: [i32; 100] = [
    0,  0,   1,   2,   3,   5,   7,   9,  12,  15,
  18,  22,  26,  30,  35,  39,  44,  50,  56,  62,
  68,  75,  82,  85,  89,  97, 105, 113, 122, 131,
 140, 150, 169, 180, 191, 202, 213, 225, 237, 248,
 260, 272, 283, 295, 307, 319, 330, 342, 354, 366,
 377, 389, 401, 412, 424, 436, 448, 459, 471, 483,
 494, 500, 500, 500, 500, 500, 500, 500, 500, 500,
 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
 500, 500, 500, 500, 500, 500, 500, 500, 500, 500
];

