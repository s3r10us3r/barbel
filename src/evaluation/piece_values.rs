use crate::{
    bitboard_helpers::pop_lsb,
    board::{board::Board, piece_set::PieceSet},
    constants::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK, WHITE},
};

use super::phase::interp_phase;

struct PieceValues {
    values: [i32; 6],
}

//NONE, PAWN, KNIGHT, BISHOP, ROOK, QUEEN
const MIDGAME_PIECE_VALUES: PieceValues = PieceValues{values: [0,100,300,350,455,900]};
const ENDGAME_PIECE_VALUES: PieceValues = PieceValues{values: [0,150,300,350,550,1000]};

pub fn evaluate_pieces(board: &Board, phase: i32) -> i32 {
    let midgame_sum = evaluate_pieces_w_vals(board.get_pieces(board.us), MIDGAME_PIECE_VALUES)
        - evaluate_pieces_w_vals(board.get_pieces(board.enemy), MIDGAME_PIECE_VALUES);
    let endgame_sum = evaluate_pieces_w_vals(board.get_pieces(board.us), ENDGAME_PIECE_VALUES)
        - evaluate_pieces_w_vals(board.get_pieces(board.enemy), ENDGAME_PIECE_VALUES);
    interp_phase(midgame_sum, endgame_sum, phase)
}

fn evaluate_pieces_w_vals(piece_set: &PieceSet, piece_values: PieceValues) -> i32 {
    let mut res = piece_set.get_pawns().count_ones() as i32 * piece_values.values[PAWN as usize];
    res += piece_set.get_knights().count_ones() as i32 * piece_values.values[KNIGHT as usize];
    res += piece_set.get_bishops().count_ones() as i32 * piece_values.values[BISHOP as usize];
    res += piece_set.get_rooks().count_ones() as i32 * piece_values.values[ROOK as usize];
    res += piece_set.get_queens().count_ones() as i32 * piece_values.values[QUEEN as usize];
    res
}

pub fn simplified_king_eval_mid(piece_set: &PieceSet) -> i32 {
    let color = piece_set.get_color();
    let mut king = piece_set.get_king();
    let mut k_i = pop_lsb(&mut king);
    if color == WHITE {
        let col = 7 - k_i / 8;
        k_i = k_i % 8 + col * 8;
    }
    return KING_PST_MID[k_i];
}

pub fn simplified_king_eval_end(piece_set: &PieceSet) -> i32 {
    let color = piece_set.get_color();
    let mut king = piece_set.get_king();
    let mut k_i = pop_lsb(&mut king);
    if color == WHITE {
        let col = 7 - k_i / 8;
        k_i = k_i % 8 + col * 8;
    }
    return KING_PST_END[k_i];
}

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
