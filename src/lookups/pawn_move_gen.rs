use crate::{bitboard_helpers::pop_lsb, constants::{BISHOP, FILEA, FILEH, KNIGHT, QUEEN, RANK1, RANK2, RANK7, RANK8, ROOK, WHITE}, lookups::simple_lookups::MoveGenerator, moving::{move_generation::MoveList, mv::Move}, position::{board::Board, piece_set::PieceSet}};

impl MoveGenerator {
    pub fn gen_pawn_moves(&self, move_list: &mut MoveList, board: &Board) {
        let (us, enemy) = board.get_piecesets();
        let occ = board.get_occupancy();
        let pawns = us.get_pawns();
        let enemies = enemy.get_all();
        if board.us == WHITE {
            self.generate_white_single_push(move_list, pawns, occ);
            self.generate_white_double_push(move_list, pawns, occ);
            self.generate_white_captures(move_list, pawns, enemies);
        } else {
            self.generate_black_single_push(move_list, pawns, occ);
            self.generate_black_double_push(move_list, pawns, occ);
            self.generate_black_captures(move_list, pawns, enemies);
        }
    }

    pub fn gen_en_passant(&self, move_list: &mut MoveList, board: &Board) {
        let file = board.get_state().get_en_passant_file();
        if file == 0 {
            return;
        }

        let us = board.get_ally_pieces();
        let pawns = us.get_pawns();
        let file = file - 1;
        let (cap_sq, target_sq) = if board.us == WHITE {
            (32 + file, 40 + file)
        } else {
            (24 + file, 16 + file)
        };
        let cap_mask = 1u64 << cap_sq;
        let pawn_mask = ((cap_mask & !FILEH) << 1) | ((cap_mask & !FILEA) >> 1);
        let mut move_mask = pawn_mask & pawns;
        while move_mask != 0 {
            let start_sq = pop_lsb(&mut move_mask);
            move_list.push_move(Move::new_en_passant(start_sq as u16, target_sq as u16));
        }
    }
}

impl MoveGenerator {
    #[inline(always)]
    fn generate_white_single_push(&self, move_list: &mut MoveList, pawns: u64, occ: u64) {
        let mut mask = (pawns << 8) & !occ;
        let mut promotions = mask & RANK8;
        mask &= !promotions;
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let start = target - 8;
            move_list.push_move(Move::new_quiet(start as u16, target as u16));
        }
        while promotions != 0 {
            let target = pop_lsb(&mut promotions);
            let start = target - 8;
            self.generate_promotions(move_list, start as u16, target as u16);
        }
    }

    #[inline(always)]
    fn generate_white_double_push(&self, move_list: &mut MoveList, pawns: u64, occ: u64) {
        let pawns = pawns & RANK2;
        let mut mask = (((pawns << 8) & !occ) << 8) & !occ;
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let start = target - 16;
            move_list.push_move(Move::new_double_pawn_push(start as u16, target as u16));
        }
    }

    #[inline(always)]
    fn generate_white_captures(&self, move_list: &mut MoveList, pawns: u64, enemies: u64) {
        let mut left_captures = ((pawns & !FILEA) << 7) & enemies;
        let mut right_captures = ((pawns & !FILEH) << 9) & enemies;

        let mut left_capture_promotions = left_captures & RANK8;
        left_captures &= !left_capture_promotions;

        let mut right_capture_promotions = right_captures & RANK8;
        right_captures &= !right_capture_promotions;

        while left_captures != 0 {
            let target = pop_lsb(&mut left_captures);
            let start = target - 7;
            move_list.push_move(Move::new_capture(start as u16, target as u16));
        }

        while right_captures != 0 {
            let target = pop_lsb(&mut right_captures);
            let start = target - 9;
            move_list.push_move(Move::new_capture(start as u16, target as u16));
        }

        while left_capture_promotions != 0 {
            let target = pop_lsb(&mut left_capture_promotions);
            let start = target - 7;
            self.generate_promotion_captures(move_list, start as u16, target as u16);
        }

        while right_capture_promotions != 0 {
            let target = pop_lsb(&mut right_capture_promotions);
            let start = target - 9;
            self.generate_promotion_captures(move_list, start as u16, target as u16);
        }
    }


    #[inline(always)]
    fn generate_promotions(&self, move_list: &mut MoveList, start: u16, target: u16) {
        move_list.push_move(Move::new_promotion(start, target, QUEEN));
        move_list.push_move(Move::new_promotion(start, target, ROOK));
        move_list.push_move(Move::new_promotion(start, target, KNIGHT));
        move_list.push_move(Move::new_promotion(start, target, BISHOP));
    }

    #[inline(always)]
    fn generate_promotion_captures(&self, move_list: &mut MoveList, start: u16, target: u16) {
        move_list.push_move(Move::new_promotion_capture(start, target, QUEEN));
        move_list.push_move(Move::new_promotion_capture(start, target, ROOK));
        move_list.push_move(Move::new_promotion_capture(start, target, KNIGHT));
        move_list.push_move(Move::new_promotion_capture(start, target, BISHOP));
    }
}


impl MoveGenerator {
    #[inline(always)]
    fn generate_black_single_push(&self, move_list: &mut MoveList, pawns: u64, occ: u64) {
        let mut mask = (pawns >> 8) & !occ;
        let mut promotions = mask & RANK1;
        mask &= !promotions;
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let start = target + 8;
            move_list.push_move(Move::new_quiet(start as u16, target as u16));
        }
        while promotions != 0 {
            let target = pop_lsb(&mut promotions);
            let start = target + 8;
            self.generate_promotions(move_list, start as u16, target as u16);
        }
    }

    #[inline(always)]
    fn generate_black_double_push(&self, move_list: &mut MoveList, pawns: u64, occ: u64) {
        let pawns = pawns & RANK7;
        let mut mask = (((pawns >> 8) & !occ) >> 8) & !occ;
        while mask != 0 {
            let target = pop_lsb(&mut mask);
            let start = target + 16;
            move_list.push_move(Move::new_double_pawn_push(start as u16, target as u16));
        }
    }

    #[inline(always)]
    fn generate_black_captures(&self, move_list: &mut MoveList, pawns: u64, enemies: u64) {
        let mut left_captures = ((pawns & !FILEA) >> 9) & enemies;
        let mut right_captures = ((pawns & !FILEH) >> 7) & enemies;

        let mut left_capture_promotions = left_captures & RANK1;
        left_captures &= !left_capture_promotions;

        let mut right_capture_promotions = right_captures & RANK1;
        right_captures &= !right_capture_promotions;

        while left_captures != 0 {
            let target = pop_lsb(&mut left_captures);
            let start = target + 9;
            move_list.push_move(Move::new_capture(start as u16, target as u16));
        }

        while right_captures != 0 {
            let target = pop_lsb(&mut right_captures);
            let start = target + 7;
            move_list.push_move(Move::new_capture(start as u16, target as u16));
        }

        while left_capture_promotions != 0 {
            let target = pop_lsb(&mut left_capture_promotions);
            let start = target + 9;
            self.generate_promotion_captures(move_list, start as u16, target as u16);
        }

        while right_capture_promotions != 0 {
            let target = pop_lsb(&mut right_capture_promotions);
            let start = target + 7;
            self.generate_promotion_captures(move_list, start as u16, target as u16);
        }
    }
}
