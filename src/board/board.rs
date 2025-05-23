use crate::constants::{BLACK, WHITE};

use super::{board_state::BoardState, piece_set::PieceSet, zobrist_hashing::ZobristHasher};

pub struct Board {
    pub players: [PieceSet; 2],
    pub us: usize,
    pub enemy: usize,
    pub hasher: ZobristHasher,
    board_state: BoardState,
    state_stack: Vec<BoardState>,
    hash_stack: Vec<u64>,
    checkers: u64,
    occ: u64,
}

impl Board {
    pub fn new(hasher: ZobristHasher) -> Board {
        Board {
            players: [PieceSet::new(BLACK), PieceSet::new(WHITE)],
            us: WHITE,
            hasher,
            enemy: BLACK,
            board_state: BoardState::new(),
            state_stack: vec![],
            hash_stack: vec![],
            checkers: 0,
            occ: 0,
        }
    }

    pub fn set_state(&mut self, new_state: BoardState) {
        self.board_state = new_state;
    }

    #[inline]
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

    pub fn push_hash(&mut self) {
        let hash = self.hasher.get_hash();
        self.hash_stack.push(hash);
    }

    pub fn pop_hash(&mut self) {
        let hash_o = self.hash_stack.pop();
        if let Some(hash) = hash_o {
            self.hasher.set_hash(hash);
        }
    }

    pub fn set_side_to_move(&mut self, color: usize) {
        self.us = color;
    }

    #[inline]
    pub fn get_pieces(&self, color: usize) -> &PieceSet {
        &self.players[color]
    }

    #[inline]
    pub fn get_occupancy(&self) -> u64 {
        self.occ
    }

    pub fn compute_occ_and_checkers(&mut self) {
        self.compute_occupancy();
        self.compute_checkers();
    }

    fn compute_occupancy(&mut self) {
        self.occ = self.players[WHITE].get_all() | self.players[BLACK].get_all();
    }

    fn compute_checkers(&mut self) {
        self.checkers = self.attackers_to_exist(
            self.players[self.us].get_king(),
            self.get_occupancy(),
            self.enemy,
        );
    }

    pub fn get_checkers(&self) -> u64 {
        self.checkers
    }

    pub fn get_hash(&self) -> u64 {
        self.hasher.get_hash()
    }

    pub fn set_hasher(&mut self, hasher: ZobristHasher) {
        self.hasher = hasher;
    }
}
