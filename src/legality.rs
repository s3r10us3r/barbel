use crate::{board::Board, constants::WHITE, lookups::BB_BETWEEN, mv::Move};

impl Board {
    pub fn is_legal(&self, mv: &Move) -> bool {
        let king_i = self.players[self.us].get_king().trailing_zeros() as u16;

        if mv.is_en_passant() {
            self.is_en_passant_legal(mv)
        } else if mv.is_kingside_castle() {
            self.is_kingside_castle_legal(mv)
        } else if mv.is_queenside_castle() {
            self.is_queenside_castle_legal(mv)
        } else if mv.get_start_field() == king_i {
            self.is_king_move_legal(mv)
        } else {
            self.is_normal_move_legal(mv)
        }
    }

    pub fn is_en_passant_legal(&self, mv: &Move) -> bool {
        let occ = self.get_occupancy();
        let cap_sq = if self.us == WHITE {
            (mv.get_target_field() - 8) as usize
        } else {
            (mv.get_target_field() + 8) as usize
        };
        let king_sq = self.get_pieces(self.us).get_king().trailing_zeros() as usize;
        let start_bb: u64 = 1 << mv.get_start_field();
        (BB_BETWEEN[cap_sq][king_sq] & !start_bb & occ).count_ones() > 0
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
        for i in (target + 1)..(start + 1) {
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

    pub fn is_normal_move_legal(&self, mv: &Move) -> bool {
        let start_bb = 1 << mv.get_start_field();
        let target_bb = 1 << mv.get_target_field();
        let occ = self.get_occupancy();
        let temp_occ = (occ & !start_bb) | target_bb;
        self.attackers_to_exist(start_bb, temp_occ, self.enemy) & !target_bb == 0
    }
}
