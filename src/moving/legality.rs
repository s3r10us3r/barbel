use crate::{bitboard_helpers::*, constants::WHITE, moving::{move_generation::MoveGenerator, move_list::MoveList, mv::Move}, position::board::{self, Board}};

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
                if !self.is_king_move_legal(mv, board) {
                    move_list.remove(i);
                }
            } else if mv.is_en_passant() {
                if !self.is_en_passant_legal_when_check(board, mv, checker) {
                    move_list.remove(i);
                }
            } else if mv.get_target_bb() & target_mask == 0 || !self.is_normal_move_legal(mv, board) {
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
                && !self.is_legal(mv, board)
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
        let king_sq = get_lsb(&king_bb);

        let mut snipers = 
            (self.get_rook_attacks(king_sq, enemies) & enemy_piece_set.get_orthogonals()) |
            (self.get_bishop_attacks(king_sq, enemies) & enemy_piece_set.get_diagonals());
        let mut pinns = 0;
        while snipers != 0 {
            let snip_sq = pop_lsb(&mut snipers);
            pinns |= self.get_bb_between(king_sq, snip_sq) & allies;
        }
        pinns
    }


    pub fn is_legal(&self, mv: &Move, board: &Board) -> bool {
        let king = board.players[board.us].get_king();
        let king_i = get_lsb(&king);

        if mv.is_en_passant() {
            self.is_en_passant_legal(mv, board)
        } else if mv.is_kingside_castle() {
            self.is_kingside_castle_legal(mv, board)
        } else if mv.is_queenside_castle() {
            self.is_queenside_castle_legal(mv, board)
        } else if mv.get_start_field() == king_i {
            self.is_king_move_legal(mv, board)
        } else {
            self.is_normal_move_legal(mv, board)
        }
    }

    pub fn is_en_passant_legal(&self, mv: &Move, board: &Board) -> bool {
        let cap_sq = if board.us == WHITE {
            mv.get_target_field() - 8
        } else {
            mv.get_target_field() + 8
        };
        let king = board.get_pieces(board.us).get_king();
        let start_bb: u64 = mv.get_start_bb();
        let target_bb: u64 = mv.get_target_bb();
        let mut occ = board.get_occupancy();
        let cap_bb = 1 << cap_sq;
        occ &= !(start_bb | cap_bb);
        occ |= target_bb;
        let attackers = self.attackers_to_exist(board, king, occ, board.enemy);
        attackers == 0
    }

    pub fn is_kingside_castle_legal(&self, mv: &Move, board: &Board) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in start..(target + 1) {
            if self.attackers_to_exist(board, 1 << i, board.get_occupancy(), board.enemy) != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_queenside_castle_legal(&self, mv: &Move, board: &Board) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in target..(start + 1) {
            if self.attackers_to_exist(board, 1 << i, board.get_occupancy(), board.enemy) != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_king_move_legal(&self, mv: &Move, board: &Board) -> bool {
        let occ = board.get_occupancy();
        let king_bb = board.players[board.us].get_king();
        let target = mv.get_target_field();
        let temp_occ = occ & !king_bb;
        self.attackers_to_exist(board, 1 << target, temp_occ, board.enemy) == 0
    }

    pub fn is_normal_move_legal(&self, mv: &Move, board: &Board) -> bool {
        let king = board.players[board.us].get_king();
        let start_bb = 1 << mv.get_start_field();
        let target_bb = 1 << mv.get_target_field();
        let occ = board.get_occupancy();
        let temp_occ = (occ & !start_bb) | target_bb;
        let mut attacker = self.attackers_to_exist(board, king, temp_occ, board.enemy) & !target_bb;
        attacker &= !target_bb;
        attacker == 0
    }

    pub fn is_en_passant_legal_when_check(&self, board: &Board, mv: &Move, checker: u64) -> bool {
        let cap_bb = if board.us == WHITE {
            mv.get_target_bb() >> 8
        } else {
            mv.get_target_bb() << 8
        };
        let king = board.get_pieces(board.us).get_king();
        let start_bb: u64 = mv.get_start_bb();
        let target_bb: u64 = mv.get_target_bb();
        let mut occ = board.get_occupancy();
        occ &= !(start_bb | cap_bb);
        occ |= target_bb;
        let mut attackers = self.attackers_to_exist(board, king, occ, board.enemy);
        attackers &= !(checker & cap_bb);
        attackers == 0 && checker == cap_bb
    }

}
