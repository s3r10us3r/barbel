use crate::board::board::Board;

impl Board {
    pub fn is_check(&self) -> bool {
        self.get_checkers() != 0
    }
}
