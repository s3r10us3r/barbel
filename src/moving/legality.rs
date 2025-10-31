use crate::{bitboard_helpers::*, constants::WHITE, moving::{move_generation::MoveGenerator, move_list::MoveList, mv::Move}, position::board::Board};
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
}

impl Board {
    pub fn is_legal(&self, mv: &Move) -> bool {
        let king = self.players[self.us].get_king();
        let king_i = get_lsb(&king);

        if mv.is_en_passant() {
            self.is_en_passant_legal(mv)
        } else if mv.is_kingside_castle() {
            self.is_kingside_castle_legal(mv)
        } else if mv.is_queenside_castle() {
            self.is_queenside_castle_legal(mv)
        } else if mv.get_start_field() == king_i {
            self.is_king_move_legal(mv)
        } else {
            self.is_normal_move_legal(mv, king)
        }
    }


    pub fn is_en_passant_legal(&self, mv: &Move) -> bool {
        let cap_sq = if self.us == WHITE {
            mv.get_target_field() - 8
        } else {
            mv.get_target_field() + 8
        };
        let king = self.get_pieces(self.us).get_king();
        let start_bb: u64 = mv.get_start_bb();
        let target_bb: u64 = mv.get_target_bb();
        let mut occ = self.get_occupancy();
        let cap_bb = 1 << cap_sq;
        occ &= !(start_bb | cap_bb);
        occ |= target_bb;
        let attackers = self.mg.attackers_to_exist(self, king, occ, self.enemy);
        attackers == 0
    }

    pub fn is_en_passant_legal_when_check(&self, mv: &Move, checker: u64) -> bool {
        let cap_bb = if self.us == WHITE {
            mv.get_target_bb() >> 8
        } else {
            mv.get_target_bb() << 8
        };
        let king = self.get_pieces(self.us).get_king();
        let start_bb: u64 = mv.get_start_bb();
        let target_bb: u64 = mv.get_target_bb();
        let mut occ = self.get_occupancy();
        occ &= !(start_bb | cap_bb);
        occ |= target_bb;
        let mut attackers = self.mg.attackers_to_exist(self, king, occ, self.enemy);
        attackers &= !(checker & cap_bb);
        attackers == 0 && checker == cap_bb
    }

    pub fn is_kingside_castle_legal(&self, mv: &Move) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in start..(target + 1) {
            if self.mg.attackers_to_exist(self, 1 << i, self.get_occupancy(), self.enemy) != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_queenside_castle_legal(&self, mv: &Move) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in target..(start + 1) {
            if self.mg.attackers_to_exist(self, 1 << i, self.get_occupancy(), self.enemy) != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_king_move_legal(&self, mv: &Move) -> bool {
        let occ = self.get_occupancy();
        let king_bb = self.players[self.us].get_king();
        let target = mv.get_target_field();
        let temp_occ = occ & !king_bb;
        self.mg.attackers_to_exist(self, 1 << target, temp_occ, self.enemy) == 0
    }

    pub fn is_normal_move_legal(&self, mv: &Move, king: u64) -> bool {
        let start_bb = 1 << mv.get_start_field();
        let target_bb = 1 << mv.get_target_field();
        let occ = self.get_occupancy();
        let temp_occ = (occ & !start_bb) | target_bb;
        let mut attacker = self.mg.attackers_to_exist(self, king, temp_occ, self.enemy) & !target_bb;
        attacker &= !target_bb;
        attacker == 0
    }
}
