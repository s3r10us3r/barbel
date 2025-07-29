use crate::{bitboard_helpers::pop_lsb, position::{board::Board, piece_set::PieceSet}, constants::{BISHOP, BLACK, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE}};

pub fn score_piece_squares(board: &Board) -> i32 {
    let score = score_piece_squares_phase(board, &MG_TABLE);
    if board.us == BLACK {
        -score
    } else {
        score
    }
}

fn score_piece_squares_phase(board: &Board, table: &BarbelTable) -> i32 {
    score_ps(board.get_pieces(WHITE), table, score_table_white) - 
        score_ps(board.get_pieces(BLACK), table, score_table_black)
}

fn score_ps<F>(ps: &PieceSet, table: &BarbelTable, f: F) -> i32 
where F: Fn(u64, &[i32; 64]) -> i32 {
    let mut score = f(ps.get_pawns(), &table[PAWN]);
    score += f(ps.get_knights(), &table[KNIGHT]);
    score += f(ps.get_bishops(), &table[BISHOP]);
    score += f(ps.get_rooks(), &table[ROOK]);
    score += f(ps.get_queens(), &table[QUEEN]);
    score += f(ps.get_king(), &table[KING]);
    score
}

#[inline]
fn score_table_white(mut pieces: u64, table: &[i32; 64]) -> i32 {
    let mut score = 0;
    while pieces != 0 {
        let lsb = pop_lsb(&mut pieces);
        let lsb = lsb ^ 56;
        score += table[lsb];
    }
    score
}

#[inline]
fn score_table_black(mut pieces: u64, table: &[i32; 64]) -> i32 {
    let mut score = 0;
    while pieces != 0 {
        let lsb = pop_lsb(&mut pieces);
        score += table[lsb];
    }
    score
}

type BarbelTable = [[i32; 64]; 6];

const MG_TABLE: BarbelTable = [
    MG_PAWN_TABLE,
    MG_KNIGHT_TABLE,
    MG_BISHOP_TABLE,
    MG_ROOK_TABLE,
    MG_QUEEN_TABLE,
    MG_KING_TABLE
];

const MG_PAWN_TABLE: [i32; 64] = [
      0,   0,   0,   0,   0,   0,  0,   0,
     98, 134,  61,  95,  68, 126, 34, -11,
     -6,   7,  26,  31,  65,  56, 25, -20,
    -14,  13,   6,  21,  23,  12, 17, -23,
    -27,  -2,  -5,  12,  17,   6, 10, -25,
    -26,  -4,  -4, -10,   3,   3, 33, -12,
    -35,  -1, -20, -23, -15,  24, 38, -22,
      0,   0,   0,   0,   0,   0,  0,   0,
];

const MG_KNIGHT_TABLE: [i32; 64] = [
    -167, -89, -34, -49,  61, -97, -15, -107,
     -73, -41,  72,  36,  23,  62,   7,  -17,
     -47,  60,  37,  65,  84, 129,  73,   44,
      -9,  17,  19,  53,  37,  69,  18,   22,
     -13,   4,  16,  13,  28,  19,  21,   -8,
     -23,  -9,  12,  10,  19,  17,  25,  -16,
     -29, -53, -12,  -3,  -1,  18, -14,  -19,
    -105, -21, -58, -33, -17, -28, -19,  -23,
];

const MG_BISHOP_TABLE: [i32; 64] = [
    -29,   4, -82, -37, -25, -42,   7,  -8,
    -26,  16, -18, -13,  30,  59,  18, -47,
    -16,  37,  43,  40,  35,  50,  37,  -2,
     -4,   5,  19,  50,  37,  37,   7,  -2,
     -6,  13,  13,  26,  34,  12,  10,   4,
      0,  15,  15,  15,  14,  27,  18,  10,
      4,  15,  16,   0,   7,  21,  33,   1,
    -33,  -3, -14, -21, -13, -12, -39, -21,
];

const MG_ROOK_TABLE: [i32; 64] = [
     32,  42,  32,  51, 63,  9,  31,  43,
     27,  32,  58,  62, 80, 67,  26,  44,
     -5,  19,  26,  36, 17, 45,  61,  16,
    -24, -11,   7,  26, 24, 35,  -8, -20,
    -36, -26, -12,  -1,  9, -7,   6, -23,
    -45, -25, -16, -17,  3,  0,  -5, -33,
    -44, -16, -20,  -9, -1, 11,  -6, -71,
    -19, -13,   1,  17, 16,  7, -37, -26,
];

const MG_QUEEN_TABLE: [i32; 64] = [
    -28,   0,  29,  12,  59,  44,  43,  45,
    -24, -39,  -5,   1, -16,  57,  28,  54,
    -13, -17,   7,   8,  29,  56,  47,  57,
    -27, -27, -16, -16,  -1,  17,  -2,   1,
     -9, -26,  -9, -10,  -2,  -4,   3,  -3,
    -14,   2, -11,  -2,  -5,   2,  14,   5,
    -35,  -8,  11,   2,   8,  15,  -3,   1,
     -1, -18,  -9,  10, -15, -25, -31, -50,
];

const MG_KING_TABLE: [i32; 64] = [
    -65,  23,  16, -15, -56, -34,   2,  13,
     29,  -1, -20,  -7,  -8,  -4, -38, -29,
     -9,  24,   2, -16, -20,   6,  22, -22,
    -17, -20, -12, -27, -30, -25, -14, -36,
    -49,  -1, -27, -39, -46, -44, -33, -51,
    -14, -14, -22, -46, -44, -30, -15, -27,
      1,   7,  -8, -64, -43, -16,   9,   8,
    -15,  36,  12, -54,   8, -28,  24,  14,
];
