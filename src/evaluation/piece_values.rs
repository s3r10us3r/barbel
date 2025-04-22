use crate::{
    bitboard_helpers::pop_lsb,
    board::piece_set::{self, PieceSet},
    constants::ROOK,
};

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 300;
const BISHOP_VALUE: i32 = 350;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;

pub fn evaluate_pieces(piece_set: &PieceSet) -> i32 {
    let diagonals = piece_set.get_diagonals();
    let orthogonals = piece_set.get_orthogonals();
    let mut res = piece_set.get_pawns().count_ones() as i32 * PAWN_VALUE;
    res += piece_set.get_knights().count_ones() as i32 * KNIGHT_VALUE;
    res += (diagonals & !orthogonals).count_ones() as i32 * BISHOP_VALUE;
    res += (orthogonals & !diagonals).count_ones() as i32 * ROOK_VALUE;
    res += piece_set.get_queens().count_ones() as i32 * QUEEN_VALUE;
    res
}

pub fn simplified_eval(piece_set: &PieceSet) -> i32 {
    let mut res = 0;
    res += get_piece_scores(piece_set.get_pawns(), PAWN_PST);
    res += get_piece_scores(piece_set.get_knights(), KNIGHT_PST);
    res += get_piece_scores(piece_set.get_queens(), QUEEN_PST);
    res += get_piece_scores(piece_set.get_rooks(), ROOK_PST);
    res += get_piece_scores(piece_set.get_bishops(), BISHOP_PST);
    res
}

pub fn simplified_king_eval_mid(piece_set: &PieceSet) -> i32 {
    let mut king = piece_set.get_king();
    let k_i = pop_lsb(&mut king);
    return KING_PST_MID[k_i];
}

pub fn simplified_king_eval_end(piece_set: &PieceSet) -> i32 {
    let mut king = piece_set.get_king();
    let k_i = pop_lsb(&mut king);
    return KING_PST_END[k_i];
}

fn get_piece_scores(mut piece_bb: u64, pst: [i32; 64]) -> i32 {
    let mut res = 0;
    while piece_bb != 0 {
        let i = pop_lsb(&mut piece_bb);
        res += pst[i];
    }
    res
}

const PAWN_PST: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];

const KNIGHT_PST: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_PST: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];

const ROOK_PST: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];

const QUEEN_PST: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

const KING_PST_MID: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
    -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30,
    -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0,
    10, 30, 20,
];

const KING_PST_END: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20, 30,
    30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30,
    -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30, -30, -30, -30,
    -30, -50,
];
