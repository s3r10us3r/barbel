use std::ops::Index;

use crate::attacks::diagonal_attacks_from;
use crate::attacks::orthogonal_attacks_from;
use crate::bitboard_helpers::get_lsb;
use crate::bitboard_helpers::has_more_than_one;
use crate::bitboard_helpers::pop_lsb;
use crate::position::board::Board;
use crate::constants::*;
use crate::lookups::*;
use crate::moving::mv::Move;

pub fn generate_moves(board: &Board) -> MoveList {
    let mut move_list = MoveList::new();
    move_list.reset();
    let checkers = board.get_checkers();
    if checkers == 0 {
        gen_legal_moves(&mut move_list, board);
    } else if has_more_than_one(checkers) {
        gen_evasions(&mut move_list, board);
    } else {
        gen_checked(&mut move_list, board, checkers);
    }
    move_list
}

fn gen_evasions(move_list: &mut MoveList, board: &Board) {
    let us_piece_set = board.get_pieces(board.us);
    let enemy_piece_set = board.get_pieces(board.enemy);
    let occ = board.get_occupancy();
    move_list.gen_king_moves(&board.lookup_holder, us_piece_set.get_king(), enemy_piece_set.get_all(), occ);
    for i in (0..move_list.get_count()).rev() {
        let mv = &move_list[i];
        if !board.is_king_move_legal(mv) {
            move_list.remove(i);
        }
    }
}

fn gen_checked(move_list: &mut MoveList, board: &Board, checker: u64) {
    let enemy_piece_set = board.get_pieces(board.enemy);
    let us_piece_set = board.get_pieces(board.us);
    let king = us_piece_set.get_king();
    let king_sq = get_lsb(&king);
    let checker_sq = get_lsb(&checker);
    let mut target_mask = checker;
    if (enemy_piece_set.get_diagonals() | enemy_piece_set.get_orthogonals()) & checker != 0 {
        target_mask |= board.lookup_holder.get_bb_between(king_sq, checker_sq);
    }
    let enemy_mask = enemy_piece_set.get_all();
    let occupancy = board.get_occupancy();
    move_list.gen_pawn_moves(us_piece_set.get_pawns(), enemy_mask, occupancy, board.us);
    move_list.gen_orthogonal_moves(&board.lookup_holder, us_piece_set.get_orthogonals(), enemy_mask, occupancy);
    move_list.gen_diagonal_moves(&board.lookup_holder, us_piece_set.get_diagonals(), enemy_mask, occupancy);
    move_list.gen_knight_moves(&board.lookup_holder, us_piece_set.get_knights(), enemy_mask, occupancy);
    move_list.gen_en_passant(board.us, us_piece_set.get_pawns(), board);
    move_list.gen_king_moves(&board.lookup_holder, king, enemy_mask, occupancy);
    filter_illegal_moves_when_check(move_list, board, target_mask, checker);
}

fn gen_legal_moves(move_list: &mut MoveList, board: &Board) {
    gen_pseudo_legal_moves(move_list, board);
    filter_illegal_moves(move_list, board);
}

fn filter_illegal_moves(move_list: &mut MoveList, board: &Board) {
    let pinns = get_pinned(board);
    let king = board.get_pieces(board.us).get_king();
    for i in (0..move_list.get_count()).rev() {
        let mv = &move_list[i];
        if (mv.is_en_passant()
            || mv.is_kingside_castle()
            || mv.is_queenside_castle()
            || mv.get_start_bb() & pinns != 0
            || mv.get_start_bb() & king != 0)
            && !board.is_legal(mv)
        {
            move_list.remove(i);
        } 
    }
}

fn filter_illegal_moves_when_check(
    move_list: &mut MoveList,
    board: &Board,
    target_mask: u64,
    checker: u64,
) {
    let king = board.get_pieces(board.us).get_king();
    for i in (0..move_list.get_count()).rev() {
        let mv = &move_list[i];
        if mv.get_start_bb() & king != 0 {
            if !board.is_king_move_legal(mv) {
                move_list.remove(i);
            }
        } else if mv.is_en_passant() {
            if !board.is_en_passant_legal_when_check(mv, checker) {
                move_list.remove(i);
            }
        } else if mv.get_target_bb() & target_mask == 0 || !board.is_normal_move_legal(mv, king) {
            move_list.remove(i);
        }
    }
}

fn gen_pseudo_legal_moves(move_list: &mut MoveList, board: &Board) {
    let us_piece_set = board.get_pieces(board.us);
    let enemy_piece_set = board.get_pieces(board.enemy);
    let enemy_mask = enemy_piece_set.get_all();
    let occupancy = board.get_occupancy();
    move_list.gen_pawn_moves(us_piece_set.get_pawns(), enemy_mask, occupancy, board.us);
    move_list.gen_orthogonal_moves(&board.lookup_holder, us_piece_set.get_orthogonals(), enemy_mask, occupancy);
    move_list.gen_diagonal_moves(&board.lookup_holder, us_piece_set.get_diagonals(), enemy_mask, occupancy);
    move_list.gen_knight_moves(&board.lookup_holder, us_piece_set.get_knights(), enemy_mask, occupancy);
    move_list.gen_king_moves(&board.lookup_holder, us_piece_set.get_king(), enemy_mask, occupancy);
    if board.get_state().can_castle_kingside(board.us) {
        move_list.gen_kingside_castle(board.us, occupancy);
    }
    if board.get_state().can_castle_queenside(board.us) {
        move_list.gen_queenside_castle(board.us, occupancy);
    }
    move_list.gen_en_passant(board.us, us_piece_set.get_pawns(), board);
}

fn get_pinned(board: &Board) -> u64 {
    let us_piece_set = board.get_pieces(board.us);
    let enemy_piece_set = board.get_pieces(board.enemy);
    let king_bb = us_piece_set.get_king();
    let allies = us_piece_set.get_all();
    let enemies = enemy_piece_set.get_all();

    let mut snipers = (orthogonal_attacks_from(&board.lookup_holder, king_bb, enemies)
        & enemy_piece_set.get_orthogonals())
        | (diagonal_attacks_from(&board.lookup_holder, king_bb, enemies) & enemy_piece_set.get_diagonals());
    let king_i = get_lsb(&king_bb);
    let mut pinns = 0;
    while snipers != 0 {
        let snip_sq = pop_lsb(&mut snipers) as usize;
        pinns |= board.lookup_holder.get_bb_between(king_i, snip_sq) & allies;
    }
    pinns
}

pub struct MoveList {
    moves: [Move; 218],
    count: usize,
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList {
            moves: std::array::from_fn(|_| Move::new_null_mv()),
            count: 0,
        }
    }
}

impl Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, i: usize) -> &Self::Output {
        &self.moves[i]
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn moves(&mut self) -> &mut [Move] {
        &mut self.moves
    }

    fn push_move(&mut self, mv: Move) {
        self.moves[self.count] = mv;
        self.count += 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn remove(&mut self, i: usize) {
        self.count -= 1;
        self.moves.swap(self.count, i);
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn get_move(&self, i: usize) -> &Move {
        &self.moves[i]
    }

    fn gen_queenside_castle(&mut self, color: usize, occ: u64) {
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
            self.push_move(Move::new_queenside_castle(king_start, king_target));
        }
    }

    fn gen_en_passant(&mut self, color: usize, pawns: u64, board: &Board) {
        let file = board.get_state().get_en_passant_file();
        if file == 0 {
            return;
        }
        let file = file - 1;
        let (cap_sq, target_sq) = if color == WHITE {
            (32 + file, 40 + file)
        } else {
            (24 + file, 16 + file)
        };
        let cap_mask: u64 = 1 << cap_sq;
        let pawn_mask = ((cap_mask & !FILEH) << 1) | ((cap_mask & !FILEA) >> 1);
        let mut move_mask = pawn_mask & pawns;
        while move_mask != 0 {
            let start_sq = pop_lsb(&mut move_mask);
            self.push_move(Move::new_en_passant(start_sq as u16, target_sq as u16));
        }
    }

    fn gen_kingside_castle(&mut self, color: usize, occ: u64) {
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
            self.push_move(Move::new_kingside_castle(king_start, king_target));
        }
    }

    fn gen_pawn_moves(&mut self, mut pawns: u64, enemy_mask: u64, occupancy: u64, color: usize) {
        while pawns != 0 {
            let start = pop_lsb(&mut pawns) as u16;
            let lsb_pointer = 1 << start;
            let single_move_mask;
            let double_move_mask;
            let left_capture_mask;
            let right_capture_mask;
            let double_pawn_rank;
            let promotion_rank;
            if color == WHITE {
                single_move_mask = (lsb_pointer << 8) & !occupancy;
                double_move_mask = (lsb_pointer << 16) & !occupancy;
                left_capture_mask = ((lsb_pointer & !FILEA) << 7) & enemy_mask;
                right_capture_mask = ((lsb_pointer & !FILEH) << 9) & enemy_mask;
                double_pawn_rank = 1;
                promotion_rank = 7;
            } else {
                single_move_mask = (lsb_pointer >> 8) & !occupancy;
                double_move_mask = (lsb_pointer >> 16) & !occupancy;
                left_capture_mask = ((lsb_pointer & !FILEA) >> 9) & enemy_mask;
                right_capture_mask = ((lsb_pointer & !FILEH) >> 7) & enemy_mask;
                double_pawn_rank = 6;
                promotion_rank = 0;
            }
            if single_move_mask != 0 {
                let target = single_move_mask.trailing_zeros() as u16;
                if target / 8 == promotion_rank {
                    self.gen_promotion_moves(start, target);
                } else {
                    let mv = Move::new_quiet(start, target);
                    self.push_move(mv);
                }
                if double_move_mask != 0 && start / 8 == double_pawn_rank {
                    let target = double_move_mask.trailing_zeros() as u16;
                    let mv = Move::new_double_pawn_push(start, target);
                    self.push_move(mv);
                }
            }

            if left_capture_mask != 0 {
                let target = left_capture_mask.trailing_zeros() as u16;
                if target / 8 == promotion_rank {
                    self.gen_promotion_captures(start, target);
                } else {
                    let mv = Move::new_capture(start, target);
                    self.push_move(mv);
                }
            }
            if right_capture_mask != 0 {
                let target = right_capture_mask.trailing_zeros() as u16;
                if target / 8 == promotion_rank {
                    self.gen_promotion_captures(start, target);
                } else {
                    let mv = Move::new_capture(start, target);
                    self.push_move(mv);
                }
            }
        }
    }

    fn gen_promotion_moves(&mut self, start: u16, target: u16) {
        let pieces = [KNIGHT, BISHOP, ROOK, QUEEN];
        for piece in pieces {
            self.push_move(Move::new_promotion(start, target, piece));
        }
    }

    fn gen_promotion_captures(&mut self, start: u16, target: u16) {
        let pieces = [KNIGHT, BISHOP, ROOK, QUEEN];
        for piece in pieces {
            self.push_move(Move::new_promotion_capture(start, target, piece));
        }
    }

    fn gen_knight_moves(&mut self, lookup_holder: &LookupHolder, mut knights: u64, enemy_mask: u64, occupancy: u64) {
        while knights != 0 {
            let start = knights.trailing_zeros() as usize;
            let move_mask = lookup_holder.get_knight_attacks(start);
            let quiet_mask = move_mask & !occupancy;
            let capture_mask = move_mask & enemy_mask;
            self.add_quiet_moves_from_mask(start as u16, quiet_mask);
            self.add_capture_moves_from_mask(start as u16, capture_mask);
            knights &= knights - 1;
        }
    }

    fn gen_orthogonal_moves(&mut self, lookup_holder: &LookupHolder, mut orthogonals: u64, enemy_mask: u64, occupancy: u64) {
        while orthogonals != 0 {
            let start = pop_lsb(&mut orthogonals);
            let attack_mask = lookup_holder.get_rook_attacks(start, occupancy);
            self.gen_ray_moves(start, attack_mask, occupancy, enemy_mask);
        }
    }

    fn gen_diagonal_moves(&mut self, lookup_holder: &LookupHolder, mut diagonals: u64, enemy_mask: u64, occupancy: u64) {
        while diagonals != 0 {
            let start = pop_lsb(&mut diagonals);
            let attack_mask = lookup_holder.get_bishop_attacks(start, occupancy);
            self.gen_ray_moves(start, attack_mask, occupancy, enemy_mask);
        }
    }

    fn gen_king_moves(&mut self, lookup_holder: &LookupHolder, king: u64, enemy_mask: u64, occupancy: u64) {
        let start = king.trailing_zeros() as usize;
        let move_mask = lookup_holder.get_king_attacks(start);
        let quiet_mask = move_mask & !occupancy;
        let capture_mask = move_mask & enemy_mask;
        self.add_quiet_moves_from_mask(start as u16, quiet_mask);
        self.add_capture_moves_from_mask(start as u16, capture_mask);
    }

    fn gen_ray_moves(&mut self, start: usize, attack_mask: u64, occupancy: u64, enemy_mask: u64) {
        let quiet_mask = attack_mask & !occupancy;
        self.add_quiet_moves_from_mask(start as u16, quiet_mask);
        let capture_mask = attack_mask & enemy_mask;
        self.add_capture_moves_from_mask(start as u16, capture_mask);
    }

    fn add_quiet_moves_from_mask(&mut self, start: u16, mut mask: u64) {
        while mask != 0 {
            let target = pop_lsb(&mut mask) as u16;
            self.push_move(Move::new_quiet(start, target));
        }
    }

    fn add_capture_moves_from_mask(&mut self, start: u16, mut mask: u64) {
        while mask != 0 {
            let target = pop_lsb(&mut mask) as u16;
            let mv = Move::new_capture(start, target);
            self.push_move(mv);
        }
    }
}

pub struct Iter<'a> {
    moves: &'a [Move],
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.moves.len() {
            let result = &self.moves[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl MoveList {
    pub fn iter(&self) -> Iter {
        Iter {
            moves: &self.moves[..self.count],
            index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{fen_parsing::parse_fen::parse_fen};

    use super::generate_moves ;

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
        let move_list = generate_moves(&board);
        for i in 0..move_list.get_count() {
            println!("{}", move_list[i].to_str());
        }
        assert_eq!(move_list.get_count(), expected_count);
    }
}
