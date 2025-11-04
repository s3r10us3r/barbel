use crate::position::board::Board;

impl Board {
    pub fn make_null_mv(&mut self) {
        self.push_hash();
        let state = self.get_state();
        let ep_file = state.get_en_passant_file();
        if ep_file != 0 {
            self.hasher.toggle_en_passant_file(ep_file - 1);
        }
        let mut new_state = state.clone();
        new_state.clear_en_passant_file();
        self.hasher.toggle_moving_side();
        self.push_state_stack(new_state);
        (self.us, self.enemy) = (self.enemy, self.us);
        self.compute_occ_and_checkers();
    }

    pub fn unmake_null_move(&mut self) {
        self.pop_hash();
        self.pop_state();
        (self.us, self.enemy) = (self.enemy, self.us);
        self.compute_occ_and_checkers();
    }
}
