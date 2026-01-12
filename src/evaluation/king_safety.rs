use crate::bitboard_helpers::{get_lsb, pop_lsb};
use crate::constants::{BISHOP, BLACK, KNIGHT, QUEEN, ROOK, WHITE};
use crate::evaluation::Evaluator;
use crate::moving::move_generation::MoveGenerator;
use crate::position::board::Board;
use crate::position::piece_set::PieceSet;

impl Evaluator {
    pub fn evaluate_king_safety(&self, board: &Board, mg: &MoveGenerator) -> i32 {
        let white_pieces = board.get_pieces(WHITE);
        let black_pieces = board.get_pieces(BLACK);

        let white_king = white_pieces.get_king();
        let black_king = black_pieces.get_king();
        
        let white_safety = self.score_king_zone_attacks_simp(get_lsb(&white_king), black_pieces, WHITE, board.get_occupancy(), mg);
        let black_safety = self.score_king_zone_attacks_simp(get_lsb(&black_king), white_pieces, BLACK, board.get_occupancy(), mg);

        let white_shield = self.calculate_shield(white_king, 8, white_pieces.get_pawns());
        let black_shield = self.calculate_shield(black_king, -8, black_pieces.get_pawns());

        let white_score = interp_by_my_material(white_safety + white_shield, black_pieces);
        let black_score = interp_by_my_material(black_safety + black_shield, white_pieces);

        white_score - black_score
    }

    fn calculate_shield(&self, king: u64, dir: i32, pawns: u64) -> i32 {
        let king_sq = get_lsb(&king) as i32;
        let file = king_sq % 8;
        
        if file == 3 || file == 4 {
            return 0;
        }

        let mut score = 0;
        if file > 0 { score += self.king_shield_file_pen(king_sq - 1, dir, pawns); }
        score += self.king_shield_file_pen(king_sq, dir, pawns);
        if file < 7 { score += self.king_shield_file_pen(king_sq + 1, dir, pawns); }
        score
    }

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


    fn score_king_zone_attacks_simp(&self, king_sq: usize, enemy_pieces: &PieceSet, color: usize, occ: u64, mg: &MoveGenerator) -> i32 {
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
            if attacks != 0 {
                attack_count += 1;
                attack_value += attacks.count_ones() * PIECE_ATTACK_CONSTANTS[KNIGHT];
            }
        }

        while bishops != 0 {
            let sq = pop_lsb(&mut bishops);
            let attacks = mg.get_bishop_attacks(sq, occ) & king_zone;
            if attacks != 0 {
                attack_count += 1;
                attack_value += attacks.count_ones() * PIECE_ATTACK_CONSTANTS[BISHOP];
            }
        }

        while rooks != 0 {
            let sq = pop_lsb(&mut rooks);
            let attacks = mg.get_rook_attacks(sq, occ) & king_zone;
            if attacks != 0 {
                attack_count += 1;
                attack_value += attacks.count_ones() * PIECE_ATTACK_CONSTANTS[ROOK];
            }
        }

        while queens != 0 {
            let sq = pop_lsb(&mut queens);
            let attacks = (mg.get_rook_attacks(sq, occ) | mg.get_bishop_attacks(sq, occ)) & king_zone;
            if attacks != 0 {
                attack_count += 1;
                attack_value += attacks.count_ones() * PIECE_ATTACK_CONSTANTS[QUEEN];
            }
        }

        -((attack_value * ATTACK_WEIGHT[6.min(attack_count as usize)]) as i32 / 100)
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

fn interp_by_my_material(score: i32, pieces: &PieceSet) -> i32 {
    let knights = pieces.get_knights().count_ones();
    let bishops = pieces.get_bishops().count_ones();
    let rooks = pieces.get_rooks().count_ones();
    let queens = pieces.get_queens().count_ones();
    let material_val = knights * KNIGHT_PHASE_VALUE + bishops * BISHOP_PHASE_VALUE + rooks * ROOK_PHASE_VALUE + queens * QUEEN_PHASE_VALUE;
    let material_val = material_val as i32;
    return material_val * score / MAX_MATERIAL_VALUE as i32;
}



const ONE_SQUARE_PENALTY: i32 = -10;
const TWO_SQUARE_PENALTY: i32 = -25;
const OPEN_FILE_PENALTY: i32 = -50;


const PIECE_ATTACK_CONSTANTS: [u32; 6] = [10, 20, 20, 40, 80, 0];
const ATTACK_WEIGHT: [u32; 7] = [0, 10, 40, 60, 75, 85, 100];


const KNIGHT_PHASE_VALUE: u32 = 1;
const BISHOP_PHASE_VALUE: u32 = 1;
const ROOK_PHASE_VALUE: u32 = 2;
const QUEEN_PHASE_VALUE: u32 = 4;
const MAX_MATERIAL_VALUE: u32 = KNIGHT_PHASE_VALUE * 2 + BISHOP_PHASE_VALUE * 2 + ROOK_PHASE_VALUE * 2 + QUEEN_PHASE_VALUE;
