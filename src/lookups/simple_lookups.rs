use crate::constants::*;

pub static KNIGHT_LOOKUP: [u64; 64] = compute_knight_lookup();
pub static RAY_LOOKUP: [[u64; 64]; 8] = compute_ray_lookup();
pub static KING_LOOKUP: [u64; 64] = compute_king_lookup();
pub static PAWN_ATTACKS_TO: [[u64; 64]; 2] = compute_pawn_lookup();
pub static BB_BETWEEN: [[u64; 64]; 64] = compute_between_bb_lookup();

const fn compute_knight_lookup() -> [u64; 64] {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        let bitboard: u64 = 1 << i;
        let mut attacks: u64 = 0;
        attacks |= (bitboard & !FILEA & !FILEB) << 6;
        attacks |= (bitboard & !FILEA & !FILEB) >> 10;
        attacks |= (bitboard & !FILEA) << 15;
        attacks |= (bitboard & !FILEA) >> 17;
        attacks |= (bitboard & !FILEG & !FILEH) << 10;
        attacks |= (bitboard & !FILEG & !FILEH) >> 6;
        attacks |= (bitboard & !FILEH) << 17;
        attacks |= (bitboard & !FILEH) >> 15;
        table[i] = attacks;
        i += 1;
    }
    table
}

const fn compute_ray_lookup() -> [[u64; 64]; 8] {
    let bit_clears = [
        RANK8,
        FILEH | RANK8,
        FILEH,
        FILEH | RANK1,
        RANK1,
        FILEA | RANK1,
        FILEA,
        FILEA | RANK8,
    ];
    let shifts = [8, 9, 1, -7, -8, -9, -1, 7];

    let mut table: [[u64; 64]; 8] = [[0; 64]; 8];
    let mut i = 0;
    while i < 64 {
        let mut dir = 0;
        while dir < 8 {
            let mut pointer: u64 = 1 << i;
            let mut map: u64 = 0;
            while pointer != 0 {
                pointer &= !bit_clears[dir];
                let shift: i32 = shifts[dir];
                if shift > 0 {
                    pointer <<= shift.abs();
                } else {
                    pointer >>= shift.abs();
                }
                map |= pointer;
            }
            table[dir][i] = map;
            dir += 1;
        }
        i += 1;
    }
    table
}

const fn compute_king_lookup() -> [u64; 64] {
    let mut i = 0;
    let mut result: [u64; 64] = [0; 64];
    while i < 64 {
        let king_bit = 1 << i;
        let mut map = 0;
        map |= (king_bit & !FILEH) << 1;
        map |= (king_bit & !(RANK8 | FILEH)) << 9;
        map |= (king_bit & !RANK8) << 8;
        map |= (king_bit & !(RANK8 | FILEA)) << 7;

        map |= (king_bit & !FILEA) >> 1;
        map |= (king_bit & !(RANK1 | FILEA)) >> 9;
        map |= (king_bit & !RANK1) >> 8;
        map |= (king_bit & !(RANK1 | FILEH)) >> 7;
        result[i] = map;
        i += 1;
    }
    result
}

const fn compute_pawn_lookup() -> [[u64; 64]; 2] {
    let mut result: [[u64; 64]; 2] = [[0; 64]; 2];
    let mut i = 0;
    while i < 64 {
        let bb: u64 = 1 << i;
        result[WHITE][i] = ((bb & !(FILEA | RANK1)) >> 9) | ((bb & !(FILEH | RANK1)) >> 7);
        result[BLACK][i] = ((bb & !(FILEA | RANK8)) << 7) | ((bb & !(FILEH | RANK8)) << 9);
        i += 1;
    }
    result
}

const fn compute_between_bb_lookup() -> [[u64; 64]; 64] {
    let mut result: [[u64; 64]; 64] = [[0; 64]; 64];
    let mut i: usize = 0;
    while i < 64 {
        let mut j: usize = 0;
        while j < 64 {
            if j == i {
                j += 1;
                continue;
            }
            let diff = if i / 8 == j / 8 {
                1
            } else if i % 8 == j % 8 {
                8
            } else if i.abs_diff(j) % 9 == 0 {
                9
            } else if i.abs_diff(j) % 7 == 0 {
                7
            } else {
                j += 1;
                continue;
            };
            let (mut s, e) = if i < j { (i + diff, j) } else { (j + diff, i) };
            let mut bb: u64 = 0;
            while s < e {
                bb |= 1 << s;
                s += diff;
            }
            result[i][j] = bb;
            j += 1;
        }
        i += 1;
    }
    result
}
