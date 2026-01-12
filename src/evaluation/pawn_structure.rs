use crate::bitboard_helpers:: pop_lsb;
use crate::constants::*;
use crate::evaluation::phase::interp_phase;
use crate::evaluation::preliminary::PreEvalResult;
use crate::evaluation::Evaluator;
use crate::moving::move_generation::pawn_attacks_all;
use crate::position::board::Board;

const K: usize = 16;

#[derive(Clone)]
pub struct PawnEvalHashEntry {
    pub mg_score: i32,
    pub eg_score: i32,
    pub white_pawns: u64,
    pub black_pawns: u64
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

    pub fn store(&mut self, white_pawns: u64, black_pawns: u64, entry: PawnEvalHashEntry) {
        let hash = hash_pawns(white_pawns, black_pawns);
        let idx = hash & self.mask;
        self.table[idx as usize] = Some(entry);
    }

    pub fn probe(&self, white_pawns: u64, black_pawns: u64) -> Option<PawnEvalHashEntry> {
        let hash = hash_pawns(white_pawns, black_pawns);
        let idx = hash & self.mask;
        let entry = &self.table[idx as usize];
        match entry {
            Some(entry) if entry.white_pawns == white_pawns && entry.black_pawns == black_pawns => {
                Some(entry.clone())
            }
            _ => None
        }
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


impl Evaluator {
    //this has to be done FIRST
    pub fn score_pawns(&mut self, board: &Board, pre_eval: &PreEvalResult) -> i32 {
        let white_pieces = board.get_pieces(WHITE);
        let black_pieces = board.get_pieces(BLACK);

        let white_pawns = white_pieces.get_pawns();
        let black_pawns= black_pieces.get_pawns();

        if let Some(entry) = self.pawn_hash.probe(white_pawns, black_pawns) {
            interp_phase(entry.mg_score, entry.eg_score, pre_eval.phase)
        } else {
            let (white_score_mg, white_score_eg) = score_pawns_side(WHITE, white_pawns, black_pawns);
            let (black_score_mg, black_score_eg) = score_pawns_side(BLACK, black_pawns, white_pawns);
            let mg_score = white_score_mg - black_score_mg;
            let eg_score = white_score_eg - black_score_eg;
            let new_entry = PawnEvalHashEntry {mg_score, eg_score, white_pawns, black_pawns};
            self.pawn_hash.store(white_pawns, black_pawns, new_entry);
            interp_phase(mg_score, eg_score, pre_eval.phase)
        }
    }
}

fn score_pawns_side(color: usize, pawns: u64, enemy_pawns: u64) -> (i32, i32) {
    let (passed_pawn_score_mg, passed_pawn_score_eg) = score_passed_pawns(color, pawns, enemy_pawns);
    let (isolated_pawns_score_mg, isolated_pawns_score_eg) = score_isolated_pawns(pawns);
    let (doubled_pawns_score_mg, doubled_pawns_score_eg) = score_doubled_pawns(pawns);
    let backwards_pawn_score = score_backwards_pawns(color,pawns, enemy_pawns);
    let connected_pawns_score = score_connected_pawns(pawns, color);
    let mg_score = passed_pawn_score_mg + isolated_pawns_score_mg + doubled_pawns_score_mg + backwards_pawn_score + connected_pawns_score;
    let eg_score = passed_pawn_score_eg + isolated_pawns_score_eg + doubled_pawns_score_eg + backwards_pawn_score + connected_pawns_score;
    (mg_score, eg_score)
}

fn score_passed_pawns(color: usize, mut pawns: u64, enemy_pawns: u64) -> (i32, i32) {
    let mut midgame_score = 0;
    let mut endgame_score = 0;
    while pawns != 0 {
        let pawn = pop_lsb(&mut pawns);
        let passed_rank = passed_rank(color, pawn, enemy_pawns);
        midgame_score += PASSED_PAWN_SCORE_MG[color][passed_rank];
        endgame_score += PASSED_PAWN_SCORE_EG[color][passed_rank];
    }
    (midgame_score, endgame_score)
}

#[inline]
fn passed_rank(color: usize, pawn: usize, enemy_pawns: u64) -> usize {
    if PAWN_FRONT[color][pawn] & enemy_pawns == 0 {
        pawn / 8
    } else {
        0
    }
}

fn score_doubled_pawns(pawns: u64) -> (i32, i32) {
    let cnt = count_doubled_pawns(pawns) as i32;
    let mg_score = DOUBLED_PAWN_PENALTY_MG * cnt;
    let eg_score = DOUBLED_PAWN_PENALTY_EG * cnt;
    (mg_score, eg_score)
}

#[inline]
fn count_doubled_pawns(pawns: u64) -> u32 {
    let mut cnt = 0;
    for file in FILES {
        let file_pawns = file & pawns;
        let ones = file_pawns.count_ones();
        cnt += ones;
    }
    cnt
}

fn score_isolated_pawns(pawns: u64) -> (i32, i32) {
    let cnt = count_isolated_pawns(pawns);
    let mg_score = cnt * ISOLATED_PAWN_PENALTY_MG;
    let eg_score = cnt * ISOLATED_PAWN_PENALTY_EG;
    (mg_score, eg_score)
}

fn count_isolated_pawns(pawns: u64) -> i32 {
    let mut cnt = 0;
    let mut pop_pawns = pawns;
    while pop_pawns != 0 {
        let pawn = pop_lsb(&mut pop_pawns);
        if is_isolated(pawn, pawns) {
            cnt += 1;
        }
    }
    cnt
}

fn score_backwards_pawns(color: usize, pawns: u64, enemy_pawns: u64) -> i32 {
    let backwards_pawns_cnt = count_backwards_pawns(color, pawns, enemy_pawns);
    backwards_pawns_cnt * BACKWARDS_PAWN_PENALTY
}

fn count_backwards_pawns(color: usize, pawns: u64, enemy_pawns: u64) -> i32 {
    if color == WHITE {
        let mut pop_pawns = pawns;
        let enemy_pawn_attacks = pawn_attacks_all(enemy_pawns, BLACK);
        let mut res = 0;
        while pop_pawns != 0 {
            let pawn = pop_lsb(&mut pop_pawns);
            let pawn_bb = 1 << pawn;
            let behind = PAWN_FRONT[BLACK][pawn];
            let stop_square = pawn_bb << 8;
            if !is_in_phalanx(pawn_bb, pawns) && pawns & behind == 0 && enemy_pawn_attacks & stop_square != 0 {
                res += 1;
            }
        }
        res
    } else {
        let mut pop_pawns = pawns;
        let enemy_pawn_attacks = pawn_attacks_all(enemy_pawns, WHITE);
        let mut res = 0;
        while pop_pawns != 0 {
            let pawn = pop_lsb(&mut pop_pawns);
            let pawn_bb = 1 << pawn;
            let behind = PAWN_FRONT[WHITE][pawn];
            let stop_square = pawn_bb >> 8;
            if !is_in_phalanx(pawn_bb, pawns) && pawns & behind == 0 && enemy_pawn_attacks & stop_square != 0 {
                res += 1;
            }
        }
        res
    }
}

#[inline]
fn is_isolated(pawn: usize, pawns: u64) -> bool {
    let file = pawn % 8;
    let mask = if file == 0 {
        FILES[file + 1]
    } else if file == 7 {
        FILES[file - 1]
    } else {
        FILES[file + 1] | FILES[file - 1]
    };

    pawns & mask == 0
}

//this function scores both phalanx and pawn chains
fn score_connected_pawns(pawns: u64, color: usize) -> i32 {
    let mut score = 0;
    let mut pawns_pop = pawns;
    while pawns_pop != 0 {
        let pawn = pop_lsb(&mut pawns_pop);
        let pawn_bb = 1u64 << pawn;
        if is_in_phalanx(pawn_bb, pawns) || is_in_chain(pawn_bb, pawns) {
            score += PAWN_CHAIN_SCORE[color][pawn / 8];
        }
    }
    score
}

fn is_in_phalanx(pawn: u64, pawns: u64) -> bool {
    let relevant_squares = 
        (pawn & !FILEA) >> 1 |
        (pawn & !FILEH) << 1;
    (relevant_squares & pawns) != 0
}

fn is_in_chain(pawn: u64, pawns: u64) -> bool {
    let relevant_squares = 
        (pawn & !FILEA) << 7 |
        (pawn & !FILEA) >> 9 |
        (pawn & !FILEH) << 9 |
        (pawn & !FILEH) >> 7;
    (relevant_squares & pawns) != 0
}

const PASSED_PAWN_SCORE_MG: [[i32; 8]; 2] = [
    [0, 100, 60, 35, 20, 10, 5, 0], 
    [0, 5, 10, 20, 35, 60, 100, 0] 
];

const PASSED_PAWN_SCORE_EG: [[i32; 8]; 2] = [
    [0, 260, 140, 85, 50, 20, 10, 0],
    [0, 10, 20, 50, 85, 140, 260, 0]
];


const ISOLATED_PAWN_PENALTY_MG: i32 = -20;
const ISOLATED_PAWN_PENALTY_EG: i32 = -50;

// these values are from https://www.scribd.com/document/10151669/All-About-Doubled-Pawns
// The article itself mentions that hese are undesirable by 1/8th of a pawn so i just doubled the
// penalty for the endgame :)
const DOUBLED_PAWN_PENALTY_MG: i32 = -12;
const DOUBLED_PAWN_PENALTY_EG: i32 = -24;

//stockfish eval scores
const PAWN_CHAIN_SCORE: [[i32; 8]; 2] = [[0, 70, 25, 15, 10, 5, 0, 0], [0, 0, 5, 10, 15, 25, 70, 0]];

const BACKWARDS_PAWN_PENALTY: i32 = -20;

// bitboards representing squares in front of a pawn on its file and adjacent files
const PAWN_FRONT: [[u64; 64]; 2] = compute_pawn_front();

const fn compute_pawn_front() -> [[u64; 64]; 2] {
    let mut result = [[0u64; 64]; 2];
    let mut sq = 0usize;
    while sq < 64 {
        let file = sq % 8;

        let mut lookup = 0u64;
        let mut ptr = sq + 8;
        while ptr < 64 {
            lookup |= 1 << ptr;
            if file > 0 {
                lookup |= 1 << (ptr - 1);
            }
            if file < 7 {
                lookup |= 1 << (ptr + 1);
            }
            ptr += 8;
        }
        result[WHITE][sq] = lookup;

        if sq > 8 {
            let mut lookup = 0u64;
            let mut ptr = sq - 8;
            while ptr >= 8 {
                lookup |= 1 << ptr;
                if file > 0 {
                    lookup |= 1 << (ptr - 1);
                }
                if file < 7 {
                    lookup |= 1 << (ptr + 1);
                }
                ptr -= 8;
            }
            result[BLACK][sq] = lookup;
        }

        sq += 1;
    }
    result
}

