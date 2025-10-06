use crate::{bitboard_helpers::{pop_lsb, reverse}, constants::*, evaluation::{phase::interp_phase, preliminary::PreEvalResult, Evaluator}, position::{board::Board, piece_set::PieceSet}};

const K: usize = 16;

#[derive(Clone)]
pub struct PawnEvalHashEntry {
    pub score: i32,
    pub half_open_us: u64,
    pub half_open_enemy: u64,
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

fn hash_pawns(us: u64, enemy: u64) -> u64 {
    const C1: u64 = 0x9E3779B185EBCA87; //golden ratio multiplier
    const C2: u64 = 0xC2B2AE3D27D4EB4F; //murmurhash mixer constant
    // for pawns, first and last 8 bits are always empty
    let mut z = (us >> 8).wrapping_mul(C1) ^ (enemy >> 8).wrapping_mul(C2).rotate_left(32);
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
    pub fn score_pawns(&mut self, us: &PieceSet, enemy: &PieceSet, pre_eval_result: &PreEvalResult) -> PawnEvalHashEntry {
        let our_pawns = us.get_pawns();
        let enemy_pawns = enemy.get_pawns();
        let pawn_hash = hash_pawns(our_pawns, enemy_pawns);
        match self.pawn_hash.probe(pawn_hash) {
            Some(entry) => entry,
            None => {
                let half_open_us = pre_eval_result.half_open_files[us.get_color()];
                let half_open_enemy = pre_eval_result.half_open_files[enemy.get_color()];
                let score = score_pawns_side(us, enemy, pre_eval_result.phase) - score_pawns_side(enemy, us, pre_eval_result.phase);
                let entry = PawnEvalHashEntry{score, half_open_us, half_open_enemy};
                self.pawn_hash.store(pawn_hash, entry.clone());
                entry
            }
        }       
    }
}

const PASSED_PAWN_BONUS_MG: i32 = 50;
const PASSED_PAWN_BONUS_EG: i32 = 100;

fn score_pawns_side(us: &PieceSet, enemy: &PieceSet, phase: i32) -> i32 {
    score_passed_pawns(us, enemy, phase)
}

fn score_passed_pawns(us: &PieceSet, enemy: &PieceSet, phase: i32) -> i32 {
    let our_pawns = us.get_pawns();
    let enemy_pawns = enemy.get_pawns();
    let cnt = if us.get_color() == WHITE {
        count_passed_pawns(our_pawns, enemy_pawns)
    } else {
        count_passed_pawns(reverse(our_pawns), reverse(enemy_pawns))
    };
    cnt * interp_phase(PASSED_PAWN_BONUS_MG, PASSED_PAWN_BONUS_EG, phase)
} 

fn count_passed_pawns(mut our_pawns: u64, enemy_pawns: u64) -> i32 {
    let mut cnt = 0;
    while our_pawns != 0 {
        let idx = pop_lsb(&mut our_pawns);
        if PASSED_PAWN_MASK[idx] & enemy_pawns == 0 {
            cnt += 1;
        }
    }
    cnt
}

const PASSED_PAWN_MASK: [u64; 64] = compute_passed_pawn_masks(); 

const fn compute_passed_pawn_masks() -> [u64; 64] {
    let mut masks = [0; 64];
    let mut i = 0;
    while i < 64 {
        let file = i % 8;
        let rank =  i / 8;
        let mut j = rank + 1;
        while j != 8 {
            let mut mask = 0;
            if file != 0 {
                mask |= 1 << (j * 8 + file - 1);
            }
            mask |= 1 << (j * 8 + file);
            if file != 7 {
                mask |= 1 << (j * 8 + file + 1);
            }
            j += 1;
            masks[i] = mask;
        }
        i += 1;
    }

    masks
}
