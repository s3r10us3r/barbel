use crate::{
    board_state::BoardState,
    constants::{BLACK, WHITE},
    mv::Move,
    piece_set::PieceSet,
};

pub struct Board {
    pub white_player: PieceSet,
    pub black_player: PieceSet,
    board_state: BoardState,
    state_stack: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            white_player: PieceSet::new(WHITE),
            black_player: PieceSet::new(BLACK),
            board_state: BoardState::new(),
            state_stack: vec![],
        }
    }

    pub fn set_state(&mut self, new_state: BoardState) {
        self.board_state = new_state;
    }

    pub fn get_white_player(&mut self) -> &mut PieceSet {
        &mut self.white_player
    }

    pub fn get_black_player(&mut self) -> &mut PieceSet {
        &mut self.black_player
    }

    pub fn get_state(&self) -> BoardState {
        self.board_state.clone()
    }

    pub fn get_mut_state(&mut self) -> &mut BoardState {
        &mut self.board_state
    }

    pub fn push_state_stack(&mut self, new_state: BoardState) {
        self.state_stack.push(self.board_state.clone());
        self.board_state = new_state;
    }

    pub fn pop_state(&mut self) -> BoardState {
        let mut state = self.state_stack.pop().unwrap();
        std::mem::swap(&mut self.board_state, &mut state);
        state
    }
}
