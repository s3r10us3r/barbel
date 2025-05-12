use crate::{bitboard_helpers::pop_lsb, board::board::Board, constants::*};

pub fn score_pawn_structure(board: &Board) -> i32 {
    eval_single_pawns(board, board.us, board.enemy) - eval_single_pawns(board, board.enemy, board.us)
}

fn eval_single_pawns(board: &Board, us: usize, enemy: usize) -> i32 {
    let our_pawns = board.get_pieces(us).get_pawns();
    let enemy_pawns = board.get_pieces(enemy).get_pawns();
    let mut score = 0;

    let mut pawns_s = our_pawns;

    for i in 0..8 {
        score += coupled(pawns_s, i);
    }

    while pawns_s != 0 {
        let is = pop_lsb(&mut pawns_s);
        if is_isolated(our_pawns, is) {
            score += ISOLATED_PAWN;
        }
        if is_passed(enemy_pawns, is, us) {
            let mut rank = is / 8;
            rank = if us == BLACK {7 - rank} else {rank};
            score += score_passed(rank);
        }
    }
    score
}

#[inline]
fn score_passed(rank: usize) -> i32 {
    match rank {
        1 => 20,
        2 => 25,
        3 => 30,
        4 => 45,
        5 => 55,
        6 => 70,
        _ => panic!("Moved by out of range!")
    }
}

#[inline]
fn is_isolated(pawns: u64, is: usize) -> bool {
    pawns & NEIGHBOR_COLS[is % 8] == 0
}

#[inline]
fn is_passed(enemy_pawns: u64, is: usize, color: usize) -> bool {
    enemy_pawns & PASSED_LOOKUP[color][is] == 0
}


#[inline]
fn coupled(pawns: u64, col: usize) -> i32 {
    let p_col = pawns & COLS[col];
    let cpld = p_col.count_ones();
    if cpld < 2 {
        0
    } else if cpld == 2 {
        DOUBLED
    } else {
        TRIPLED
    }
}

//The below values are kept low deliberatly
//I added a PAWN_FACTOR to keep this more flexible
const PAWN_FACTOR: i32 = 1;
const ISOLATED_PAWN: i32 = -20 * PAWN_FACTOR;
const DOUBLED: i32 = -10 * PAWN_FACTOR;
const TRIPLED: i32 = -10 * PAWN_FACTOR;

const NEIGHBOR_COLS: [u64; 8] = [FILEB, FILEA | FILEC, FILEB | FILED, FILEC | FILEE, FILED | FILEF, FILEE | FILEG, FILEF | FILEH, FILEG];
const COLS: [u64; 8] = [FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH];

const PASSED_LOOKUP: [[u64; 64]; 2] = compute_passed_lookup();
const BACKWARDS_LOOKUP: [[u64; 64]; 2] = compute_backwards_lookup();

//mask of pawns in front of a pawn 
const fn compute_passed_lookup() -> [[u64; 64]; 2] {
    let mut res = [[0; 64]; 2];
    let mut color = 0;
    while color < 2 {
        let mut i = 8;
        while i < 56 {
            let mut row = i / 8;
            if color == BLACK {
                row -= 1;
            } else {
                row += 1;
            }

            let col = i % 8;
            let col_s= if col > 0 {col - 1} else {col};
            let col_e = if col < 7 {col + 1} else {col};
            let mut row_e = if color == BLACK { 0 } else { 7 };
            if row_e < row {let t = row; row = row_e; row_e = t};

            let mut x = col_s;
            let mut mask: u64 = 0;
            while x <= col_e {
                let mut y = row;
                while y <= row_e {
                    mask |= 1 << (y * 8 + x);
                    y += 1;
                }
                x += 1;
            }
            res[color][i] = mask;
            i += 1;
        }
        color += 1;
    }
    res
}

//a pawn is backwards when, the square in front of it is controlled 
//and is not protected by other pawns
const fn compute_backwards_lookup() -> [[u64; 64]; 2]{
    let mut res = [[0; 64]; 2];
    let mut color = 0;
    while color < 2 {
        let mut i = 8;
        while i < 56 {
            let mut row = i / 8;
            let col = i % 8;
            let col_s = if col > 0 {col - 1} else {col};
            let col_e = if col < 7 {col + 1} else {col};
            let mut row_e = if color == BLACK {7} else {0};
            if row_e < row {let t = row; row = row_e; row_e = t};

            let mut x= col_s;
            let mut mask = 0;

            while x <= col_e {
                let mut y = row;
                while y <= row_e {
                    mask |= 1 << (y * 8 + x);
                    y += 1;
                }
                x += 1;
            }
            mask &= !(1 << i);
            res[color][i] = mask;
            i += 1;
        }
        color += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::{evaluation::pawn_structure::{DOUBLED, ISOLATED_PAWN, TRIPLED}, fen_parsing::fen_parsing::parse_fen};

    use super::eval_single_pawns;

    //should find 3 isolated pawns so the score should be -3 * ISOLATED_PAWN
    #[test]
    fn should_find_isolated_pawn() {
        let board = parse_fen("rnbqkbnr/p2ppp1p/8/8/8/8/P3P2P/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = eval_single_pawns(&board, board.us, board.enemy);
        assert_eq!(score, 3 * ISOLATED_PAWN);
    }

    //should find one passed pawn on file a moved by 2
    #[test]
    fn should_find_passed_pawn() {
        let board = parse_fen("rnbqkbnr/2pppp1p/8/8/P7/1P6/8/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = eval_single_pawns(&board, board.us, board.enemy);
        assert_eq!(score, 30);
    }

    #[test]
    fn should_find_double_pawns() {
        let board = parse_fen("rnbqkbnr/2pppp1p/8/8/8/4P3/3PPP2/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = eval_single_pawns(&board, board.us, board.enemy);
        assert_eq!(score, DOUBLED);
    }

    #[test]
    fn should_find_tripled_pawns() {
        let board = parse_fen("rnbqkbnr/2pppp1p/8/8/4P3/4P3/3PPP2/RNBQKBNR w KQkq - 0 1").unwrap();
        let score = eval_single_pawns(&board, board.us, board.enemy);
        assert_eq!(score, TRIPLED);
    }
}
