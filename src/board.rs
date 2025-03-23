use crate::{
    constants::{BLACK, WHITE},
    piece_set::PieceSet,
};

pub struct Board {
    white_player: PieceSet,
    black_player: PieceSet,
    board_state: u32,
    state_stack: Vec<u32>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            white_player: PieceSet::new(),
            black_player: PieceSet::new(),
            board_state: 0,
            state_stack: vec![],
        }
    }

    pub fn set_state(&mut self, new_state: u32) {
        self.board_state = new_state;
    }

    pub fn get_white_player(&mut self) -> &mut PieceSet {
        &mut self.white_player
    }

    pub fn get_black_player(&mut self) -> &mut PieceSet {
        &mut self.black_player
    }

    pub fn get_state(&self) -> u32 {
        self.board_state
    }

    pub fn get_side_to_move(&self) -> u8 {
        (self.board_state & SIDE_TO_MOVE_MASK) as u8
    }

    pub fn set_side_to_move(&mut self, color: u8) {
        self.board_state &= !SIDE_TO_MOVE_MASK;
        self.board_state |= color as u32;
    }

    pub fn set_en_passant_file(&mut self, file: u8) {
        self.board_state &= !EN_PASSANT_MASK;
        self.board_state |= ((file + 1) as u32) << 5;
    }

    pub fn get_en_passant_file(&self) -> u32 {
        (self.board_state & EN_PASSANT_MASK) >> 5
    }

    pub fn set_castling_rights(&mut self, castling_rights: u32, color: u8) {
        if color == WHITE {
            self.board_state &= !WHITE_CASTLING_RIGHTS_MASK;
            self.board_state |= castling_rights << 3;
        } else if color == BLACK {
            self.board_state &= !BLACK_CASTLING_RIGHTS_MASK;
            self.board_state |= castling_rights << 1;
        }
    }

    pub fn get_castling_rights(&self) -> u32 {
        (self.board_state & (WHITE_CASTLING_RIGHTS_MASK | BLACK_CASTLING_RIGHTS_MASK)) >> 1
    }

    pub fn is_state_bit_set(&self, mask: u32) -> bool {
        self.board_state & mask == 0
    }

    pub fn set_state_bits(&mut self, mask: u32) {
        self.board_state |= mask;
    }

    pub fn clear_state_bits(&mut self, mask: u32) {
        self.board_state &= !mask;
    }

    pub fn set_halfmove_clock(&mut self, halfmove_clock: u32) {
        self.board_state &= !HALFMOVE_CLOCK_MASK;
        self.board_state |= halfmove_clock << 9;
    }

    pub fn get_halfmove_clock(&self) -> u32 {
        (self.board_state & HALFMOVE_CLOCK_MASK) >> 9
    }

    pub fn increase_halfmove_clock(&mut self) {
        self.board_state += 1 << 9;
    }

    pub fn clear_halfmove_clock(&mut self) {
        self.board_state &= !HALFMOVE_CLOCK_MASK;
    }

    pub fn set_move_clock(&mut self, move_clock: u32) {
        self.board_state &= !MOVE_CLOCK_MASK;
        self.board_state |= move_clock << 17;
    }

    pub fn increase_move_clock(&mut self) {
        self.board_state += 1 << 17;
    }

    pub fn get_move_clock(&self) -> u32 {
        (self.board_state & MOVE_CLOCK_MASK) >> 17
    }
}

pub const SIDE_TO_MOVE_MASK: u32 = 1;
pub const EN_PASSANT_MASK: u32 = 0b1111 << 5;
pub const BLACK_CASTLING_RIGHTS_MASK: u32 = 0b11 << 1;
pub const WHITE_CASTLING_RIGHTS_MASK: u32 = 0b11 << 3;
pub const BLACK_KINGSIDE_CASTLE_MASK: u32 = 1 << 1;
pub const BLACK_QUEENSIDE_CASTLE_MASK: u32 = 1 << 2;
pub const WHITE_KINGSIDE_CASTLE_MASK: u32 = 1 << 3;
pub const WHITE_QUEENSIDE_CASTLE_MASK: u32 = 1 << 4;
pub const HALFMOVE_CLOCK_MASK: u32 = 0xFF << 9;
pub const MOVE_CLOCK_MASK: u32 = 0xFF << 17;
