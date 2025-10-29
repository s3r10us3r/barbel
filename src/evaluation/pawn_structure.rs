use crate::{bitboard_helpers::{get_file, get_lsb, get_rank, pop_lsb, reverse}, constants::*, evaluation::{phase::interp_phase, preliminary::PreEvalResult, Evaluator}, position::{board::Board, piece_set::PieceSet}};

const K: usize = 16;

#[derive(Clone)]
pub struct PawnEvalHashEntry {
    pub score: i32,
    /*
    * each 3 bits correspond to the rank of a pawn, first 24 for white, next 24 for black, 
    */
    pub pawn_ranks: u64,
}

//option here is relatively cheap since the table is already small
//nevertheless it needs at least 8 bits of space
pub struct PawnEvalHashTable {
    table: Vec<Option<PawnEvalHashEntry>>,
    mask: u64
}

impl PawnEvalHashTable {
    pub fn new() -> Self {
        let size = 1 << K;
        PawnEvalHashTable { table: vec![None; size], mask: (size as u64) - 1 }
    }

    pub fn store(&mut self, hash: u64, entry: PawnEvalHashEntry) {
        let idx = hash & self.mask;
        self.table[idx as usize] = Some(entry);
    }

    pub fn probe(&self, hash: u64) -> Option<PawnEvalHashEntry> {
        let idx = hash & self.mask;
        self.table[idx as usize].clone()
    }
}

pub fn hash_pawns(white: u64, black: u64) -> u64 {
    const C1: u64 = 0x9E3779B185EBCA87; //golden ratio multiplier
    const C2: u64 = 0xC2B2AE3D27D4EB4F; //murmurhash mixer constant
    // for pawns, first and last 8 bits are always empty
    let mut z = (white >> 8).wrapping_mul(C1) ^ (black >> 8).wrapping_mul(C2).rotate_left(32);
    // hashing, the constants are directly from splitmix paper
    const C3: u64 = 0xBF58476D1CE4E5B9;
    const C4: u64 = 0x94D049BB133111EB; 
    z ^= z >> 30;
    z = z.wrapping_mul(C3);
    z ^= z >> 27;
    z = z.wrapping_mul(C4);
    z ^= z >> 31;
    z
}

fn compute_pawn_ranks(white_pawns: u64, black_pawns: u64) -> u64 {
    let mut result = 0u64;
    for (i, file) in FILES.iter().enumerate() {
        result |= ((white_pawns & file << i).trailing_zeros() as u64 / 8) << (3 * i);
        result |= ((black_pawns & file << i).trailing_zeros() as u64 / 8) << (24 + 3 * i);
    }
    result
}

impl Evaluator {
    //this has to be done FIRST
    pub fn score_pawns(&mut self, white_pieces: &PieceSet, black_pieces: &PieceSet) {
        let white_pawns = white_pieces.get_pawns();
        let black_pawns= black_pieces.get_pawns();

        let pawn_hash = hash_pawns(white_pawns, black_pawns);
        //TODO: THIS
    }

    fn score_pawn_shield(&mut self, king: u64, pawn_ranks: u64, color: usize) -> i32 {
        let mut score = 0;
        let kr = get_rank(king) as i32;
        let kf = get_file(king) as i32;
        if color == WHITE {
            if kf > 0 {    
                let pawn_rank = ((pawn_ranks >> (3 * (kf - 1))) & 0b111) as i32;
                let idx = kr - pawn_rank;
                if idx >= 0 {
                    score += SIDE_PS_PENALTY[idx as usize];
                }
            }

            let pawn_rank = ((pawn_ranks >> (3 * kf)) & 0b111) as i32;
            let idx = kr - pawn_rank;
            if idx >= 0 {
                score += CENTER_PS_PENALTY[idx as usize];
            }

            if kf < 7 {
                let pawn_rank = ((pawn_ranks >> (3 * (kf + 1))) & 0b111) as i32;
                let idx = kr - pawn_rank;
                if idx >= 0 {
                    score += SIDE_PS_PENALTY[idx as usize];
                }
            }
        } else {
            if kf > 0 {    
                let pawn_rank = ((pawn_ranks >> (3 * (kf + 23))) & 0b111) as i32;
                let idx = pawn_rank - kr;
                if idx >= 0 {
                    score += SIDE_PS_PENALTY[idx as usize];
                }
            }

            let pawn_rank = ((pawn_ranks >> (3 * (kf + 24))) & 0b111) as i32;
            let idx = pawn_rank - kr;
            if idx >= 0 {
                score += CENTER_PS_PENALTY[idx as usize];
            }

            if kf < 7 {
                let pawn_rank = ((pawn_ranks >> (3 * (kf + 25))) & 0b111) as i32;
                let idx = pawn_rank - kr;
                if idx >= 0 {
                    score += SIDE_PS_PENALTY[idx as usize];
                }
            }
        }
        score
    }
}

const WHITE_PAWN_RANKS: u64 = 0xffffff;
const BLACK_PAWN_RANKS: u64 = 0x000000_ffffff;

const SIDE_PS_PENALTY: [i32; 8] = [0, 0, -15, -30, -30, -30, -30, -30];
const CENTER_PS_PENALTY: [i32; 8] = [0, 0, -25, -40, -40, -40, -40, -40];
