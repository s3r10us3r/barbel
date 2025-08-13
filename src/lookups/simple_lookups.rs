use crate::{bitboard_helpers::{get_lsb, has_more_than_one, pop_lsb}, constants::*, lookups::magics::{compute_bishop_lookup, compute_rook_lookup, index_magic, BISHOP_MAGICS, BISHOP_RELEVANCY_MASKS, BISHOP_SHIFTS, ROOK_MAGICS, ROOK_RELEVANCY_MASKS, ROOK_SHIFTS}, moving::{move_generation::MoveList, mv::Move}, position::board::Board};

pub struct MoveGenerator {
    rook_lookup: Vec<Vec<u64>>,
    bishop_lookup: Vec<Vec<u64>>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self { rook_lookup: compute_rook_lookup(), bishop_lookup: compute_bishop_lookup() }
    }

    pub fn generate_moves(&self, board: &Board) -> MoveList {
        let mut move_list = MoveList::new();
        let checkers = board.get_checkers();
        if checkers == 0 {
            self.gen_legal_moves(&mut move_list, board);
        } else if has_more_than_one(checkers) {
            self.gen_evasions(&mut move_list, board);
        } else {
            self.gen_checked(&mut move_list, board, checkers);
        }
        move_list
    }

    fn gen_legal_moves(&self, move_list: &mut MoveList, board: &Board) {
        let (us, enemy) = board.get_piecesets();
        let enemy_mask = enemy.get_all();
        let occ= board.get_occupancy();

        self.gen_pawn_moves(move_list, board);
        self.gen_orthogonal_moves(move_list, us.get_orthogonals(), enemy_mask, occ);
        self.gen_diagonal_moves(move_list, us.get_diagonals(), enemy_mask, occ);
        self.gen_knight_moves(move_list, us.get_knights(), occ, enemy_mask);
        self.gen_king_moves(move_list, us.get_king(), enemy_mask, occ);
        if board.get_state().can_castle_kingside(board.us) {
            self.gen_kingside_castle(move_list, board.us, occ);
        }
        if board.get_state().can_castle_queenside(board.us) {
            self.gen_queenside_castle(move_list,board.us, occ);
        }
        self.gen_en_passant(move_list, board);

        self.filter_illegal_moves(move_list, board);
    }

    fn gen_evasions(&self, move_list: &mut MoveList, board: &Board) {
        let us = board.get_pieces(board.us);
        let enemy = board.get_pieces(board.enemy);
        let occ = board.get_occupancy();

        self.gen_king_moves(move_list, us.get_king(), enemy.get_all(), occ);
        for i in (0..move_list.get_count()).rev() {
            let mv = &move_list[i];
            if !board.is_king_move_legal(mv) {
                move_list.remove(i);
            }
        }
    }

    fn gen_checked(&self, move_list: &mut MoveList, board: &Board, checker: u64) {
        let (us, enemy) = board.get_piecesets();
        let king = us.get_king();
        let enemy_mask = enemy.get_all();
        let occ = board.get_occupancy();
        self.gen_pawn_moves(move_list, board);
        self.gen_orthogonal_moves(move_list, us.get_orthogonals(), enemy_mask, occ);
        self.gen_diagonal_moves(move_list, us.get_diagonals(), enemy_mask, occ);
        self.gen_en_passant(move_list, board);
        self.gen_knight_moves(move_list, us.get_knights(), occ, enemy_mask);
        self.gen_king_moves(move_list, king, enemy_mask, occ);
        self.filter_illegal_moves_when_check(move_list, board, checker);
    }

    fn gen_knight_moves(&self, move_list: &mut MoveList, mut knights: u64, occ: u64, enemy_mask: u64) {
        while knights != 0 {
            let start = pop_lsb(&mut knights);
            let attacks = self.get_knight_attacks(start);
            let quiet_mask = attacks & !occ;
            let capture_mask = attacks & enemy_mask;
            self.add_quiet_moves(move_list, start, quiet_mask);
            self.add_capture_moves(move_list, start, capture_mask);
        }
    } 

    fn gen_king_moves(&self, move_list: &mut MoveList, king: u64, enemy_mask: u64, occ: u64) {
        let sq = get_lsb(&king);
        let move_mask = self.get_king_attacks(sq);
        let quiet_mask = move_mask & !occ;
        let capture_mask = move_mask & enemy_mask;

        self.add_quiet_moves(move_list, sq, quiet_mask);
        self.add_capture_moves(move_list, sq, capture_mask);
    }

    fn gen_orthogonal_moves(&self, move_list: &mut MoveList, mut orthogonals: u64, enemy_mask: u64, occ: u64) {
        while orthogonals != 0 {
            let start = pop_lsb(&mut orthogonals);
            let attacks_mask = self.get_rook_attacks(start, occ);
            let quiet_mask = attacks_mask & !occ;
            let capture_mask = attacks_mask & enemy_mask;
            self.add_quiet_moves(move_list, start, quiet_mask);
            self.add_capture_moves(move_list, start, capture_mask);
        }
    }

    fn gen_diagonal_moves(&self, move_list: &mut MoveList, mut diagonals: u64, enemy_mask: u64, occ: u64) {
        while diagonals != 0 {
            let start = pop_lsb(&mut diagonals);
            let attacks_mask = self.get_bishop_attacks(start, occ);
            let quiet_mask = attacks_mask & !occ;
            let capture_mask = attacks_mask & enemy_mask;
            self.add_quiet_moves(move_list, start, quiet_mask);
            self.add_capture_moves(move_list, start, capture_mask);
        }
    }

    fn gen_kingside_castle(&self, move_list: &mut MoveList, color: usize, occ: u64) {
        let king_start: u16;
        let king_target: u16;
        let castle_mask: u64;
        if color == WHITE {
            king_start = 4;
            king_target = 6;
            castle_mask = KINGSIDE_CASTLE_MASK;
        } else {
            king_start = 4 + 56;
            king_target = 6 + 56;
            castle_mask = KINGSIDE_CASTLE_MASK << 56;
        }
        if occ & castle_mask == 0 {
            move_list.push_move(Move::new_kingside_castle(king_start, king_target));
        }
    }

    fn gen_queenside_castle(&self, move_list: &mut MoveList, color: usize, occ: u64) {
        let king_start: u16;
        let king_target: u16;
        let castle_mask: u64;
        if color == WHITE {
            king_start = 4;
            king_target = 2;
            castle_mask = QUEENSIDE_CATLE_MASK;
        } else {
            king_start = 4 + 56;
            king_target = 2 + 56;
            castle_mask = QUEENSIDE_CATLE_MASK << 56;
        }
        if occ & castle_mask == 0 {
            move_list.push_move(Move::new_queenside_castle(king_start, king_target));
        }
    }

    #[inline(always)]
    fn add_quiet_moves(&self, move_list: &mut MoveList, start: usize, mut mask: u64) {
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let mv = Move::new_quiet(start as u16, target as u16);
            move_list.push_move(mv);
        }
    }

    #[inline(always)]
    fn add_capture_moves(&self, move_list: &mut MoveList, start: usize, mut mask: u64) {
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let mv = Move::new_capture(start as u16, target as u16);
            move_list.push_move(mv);
        }
    }

    #[inline(always)]
    pub fn get_knight_attacks(&self, sq: usize) -> u64 {
        KNIGHT_LOOKUP[sq]
    }

    #[inline(always)]
    pub fn get_king_attacks(&self, sq: usize) -> u64 {
        KING_LOOKUP[sq]
    }

    #[inline(always)]
    pub fn get_pawn_attacks(&self, sq: usize, color: usize) -> u64 {
        PAWN_ATTACKS_TO[color][sq]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, sq: usize, occ: u64) -> u64 {
        let bb = occ & ROOK_RELEVANCY_MASKS[sq];
        let idx = index_magic(bb, ROOK_MAGICS[sq], ROOK_SHIFTS[sq]);
        self.rook_lookup[sq][idx]
    }

    #[inline(always)]
    pub fn get_bishop_attacks(&self, sq: usize, occ: u64) -> u64 {
        let bb = occ & BISHOP_RELEVANCY_MASKS[sq];
        let idx = index_magic(bb, BISHOP_MAGICS[sq], BISHOP_SHIFTS[sq]);
        self.bishop_lookup[sq][idx]
    }

    pub fn get_bb_between(&self, sq1: usize, sq2: usize) -> u64 {
        BB_BETWEEN[sq1][sq2]
    }
}


pub static KNIGHT_LOOKUP: [u64; 64] = compute_knight_lookup();
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


#[cfg(test)]
mod test {
    use crate::{fen_parsing::parse_fen::parse_fen, lookups::simple_lookups::MoveGenerator};


    #[test]
    fn should_have_correct_move_count_in_starting_position() {
        should_have_correct_move_count_in_pos(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            20,
        );
    }

    #[test]
    fn should_have_correct_move_count_in_kiwipete_position() {
        should_have_correct_move_count_in_pos(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            48,
        );
    }

    #[test]
    fn should_have_correct_move_count_in_position_3() {
        should_have_correct_move_count_in_pos("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ", 14);
    }

    #[test]
    fn should_have_correct_move_count_in_position_4() {
        should_have_correct_move_count_in_pos(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            6,
        );
    }

    #[test]
    fn should_have_correct_move_count_in_position_5() {
        should_have_correct_move_count_in_pos(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            44,
        );
    }

    #[test]
    fn should_have_correct_move_count_in_position_6() {
        should_have_correct_move_count_in_pos(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            46,
        );
    }

    fn should_have_correct_move_count_in_pos(pos: &str, expected_count: usize) {
        let board = parse_fen(pos).unwrap();
        let move_gen = MoveGenerator::new();
        let move_list = move_gen.generate_moves(&board);
        for i in 0..move_list.get_count() {
            println!("{}", move_list[i].to_str());
        }
        assert_eq!(move_list.get_count(), expected_count);
    }
}
