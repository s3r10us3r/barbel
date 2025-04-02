use crate::{bitboard_helpers::get_lsb, board::board::Board, constants::WHITE, moving::mv::Move};

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
        let attackers = self.attackers_to_exist(king, occ, self.enemy);
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
        let mut attackers = self.attackers_to_exist(king, occ, self.enemy);
        attackers &= !(checker & cap_bb);
        attackers == 0 && checker == cap_bb
    }

    pub fn is_kingside_castle_legal(&self, mv: &Move) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in start..(target + 1) {
            if self.attackers_to_exist(1 << i, self.get_occupancy(), self.enemy) != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_queenside_castle_legal(&self, mv: &Move) -> bool {
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        for i in target..(start + 1) {
            if self.attackers_to_exist(1 << i, self.get_occupancy(), self.enemy) != 0 {
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
        self.attackers_to_exist(1 << target, temp_occ, self.enemy) == 0
    }

    pub fn is_normal_move_legal(&self, mv: &Move, king: u64) -> bool {
        let start_bb = 1 << mv.get_start_field();
        let target_bb = 1 << mv.get_target_field();
        let occ = self.get_occupancy();
        let temp_occ = (occ & !start_bb) | target_bb;
        let mut attacker = self.attackers_to_exist(king, temp_occ, self.enemy) & !target_bb;
        attacker &= !target_bb;
        attacker == 0
    }
}
