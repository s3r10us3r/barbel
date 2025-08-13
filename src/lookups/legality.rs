use crate::bitboard_helpers::pop_lsb;
use crate::{bitboard_helpers::get_lsb, lookups::simple_lookups::MoveGenerator, moving::move_generation::MoveList, position::board::Board};
use crate::attacks::*;

impl MoveGenerator {
    pub fn filter_illegal_moves_when_check(&self, move_list: &mut MoveList, board: &Board, checker: u64) {
        let (us, enemy) = board.get_piecesets();
        let king = us.get_king();
        let king_sq = get_lsb(&king);
        let checker_sq = get_lsb(&checker);

        let mut target_mask = checker;
        if (enemy.get_diagonals() | enemy.get_orthogonals()) & checker != 0 {
            target_mask |= self.get_bb_between(king_sq, checker_sq);
        };
        for i in (0..move_list.get_count()).rev() {
            let mv = move_list.get_move(i);
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

    pub fn filter_illegal_moves(&self, move_list: &mut MoveList, board: &Board) {
        let pinns = self.get_pinned(board);
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

    fn get_pinned(&self, board: &Board) -> u64 {
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
}
