use crate::position::board::Board;

//TODO: move this
impl Board {
    pub fn is_check(&self) -> bool {
        self.get_checkers() != 0
    }
}
