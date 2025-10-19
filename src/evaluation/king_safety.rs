use crate::{bitboard_helpers::{flip_color, get_file, get_lsb, pop_lsb}, constants::{BISHOP, KNIGHT, QUEEN, ROOK, WHITE}, evaluation::{phase::interp_phase, preliminary::PreEvalResult, Evaluator}, fen_parsing::parse_to_fen, moving::move_generation::MoveGenerator, position::{board::Board, piece_set::PieceSet}};

impl Evaluator {
    pub fn evaluate_king_safety(&self, board: &Board, color: usize, pre_eval_result: &PreEvalResult) -> i32 {
        let mut pawn_shield_score = 0;
        let us = board.get_pieces(color);
        if !(board.get_state().can_castle_kingside(color) || board.get_state().can_castle_queenside(color)) {
            pawn_shield_score += self.score_pawn_shield(us.get_king(), us.get_pawns(), color);
        }
        let our_king_sq = get_lsb(&us.get_king());

        let king_safety_score = self.score_king_zone_attacks_simp(our_king_sq, board.get_pieces(flip_color(color)), &board.mg, color, board.get_occupancy());
        let score = pawn_shield_score + king_safety_score;
        interp_phase(score, 0, pre_eval_result.phase)
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


    fn score_king_zone_attacks_simp(&self, king_sq: usize, enemy_pieces: &PieceSet, mg: &MoveGenerator, color: usize, occ: u64) -> i32 {
        let king_zone = find_king_zone(king_sq, mg, color);


        let mut knights = enemy_pieces.get_knights();
        let mut rooks = enemy_pieces.get_rooks();
        let mut bishops = enemy_pieces.get_bishops();
        let mut queens= enemy_pieces.get_queens();

        let mut attack_value = 0;
        let mut attack_count = 0;

        while knights != 0 {
            let sq = pop_lsb(&mut knights);
            let attacks = mg.get_knight_attacks(sq) & king_zone;
            attack_count += 1;
            attack_value += attacks.count_ones() * ATTACK_WEIGHT[KNIGHT];
        }

        while bishops != 0 {
            let sq = pop_lsb(&mut bishops);
            let attacks = mg.get_bishop_attacks(sq, occ) & king_zone;
            attack_count += 1;
            attack_value += attacks.count_ones() * ATTACK_WEIGHT[BISHOP];
            
        }

        while rooks != 0 {
            let sq = pop_lsb(&mut rooks);
            let attacks = mg.get_rook_attacks(sq, occ) & king_zone;
            attack_count += 1;
            attack_value += attacks.count_ones() * ATTACK_WEIGHT[ROOK];
        }

        while queens != 0 {
            let sq = pop_lsb(&mut queens);
            let attacks = (mg.get_rook_attacks(sq, occ) | mg.get_bishop_attacks(sq, occ)) & king_zone;
            attack_count += 1;
            attack_value += attacks.count_ones() * ATTACK_WEIGHT[QUEEN];
        }

        -((attack_value * ATTACK_WEIGHT[attack_count as usize]) as i32 / 100)
    }
}

fn find_king_zone(king_sq: usize, mg: &MoveGenerator, color: usize) -> u64 {
    let king_attack = mg.get_king_attacks(king_sq);
    if color == WHITE {
        king_attack | (king_attack << 8)
    } else {
        king_attack | (king_attack >> 8)
    }
}

const ONE_SQUARE_PENALTY: i32 = -10;
const TWO_SQUARE_PENALTY: i32 = -25;
const OPEN_FILE_PENALTY: i32 = -50;


const PIECE_ATTACK_CONSTANTS: [u32; 6] = [0, 20, 20, 40, 80, 0];
const ATTACK_WEIGHT: [u32; 20] = [0, 50, 75, 88, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100];


