use crate::{
    board_state::BoardState,
    constants::{BLACK, WHITE},
    piece_set::PieceSet,
};

pub struct Board {
    pub players: [PieceSet; 2],
    pub us: usize,
    pub enemy: usize,
    board_state: BoardState,
    state_stack: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            players: [PieceSet::new(BLACK), PieceSet::new(WHITE)],
            us: WHITE,
            enemy: BLACK,
            board_state: BoardState::new(),
            state_stack: vec![],
        }
    }

    pub fn set_state(&mut self, new_state: BoardState) {
        self.board_state = new_state;
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

    pub fn set_size_to_move(&mut self, color: usize) {
        self.us = color;
    }

    pub fn get_pieces(&self, color: usize) -> &PieceSet {
        &self.players[color]
    }

    pub fn get_occupancy(&self) -> u64 {
        self.players[WHITE].get_all() | self.players[BLACK].get_all()
    }
}
