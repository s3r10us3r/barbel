use crate::constants::{BLACK, KING, QUEEN, WHITE};

#[derive(Clone)]
pub struct BoardState {
    value: u32,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState { value: 0 }
    }

    pub fn get_side_to_move(&self) -> u8 {
        (self.value & SIDE_TO_MOVE_MASK) as u8
    }

    pub fn set_side_to_move(&mut self, side_to_move: u8) {
        self.value = (self.value & !SIDE_TO_MOVE_MASK) | (side_to_move as u32);
    }

    pub fn flip_side_to_move(&mut self) {
        self.value ^= SIDE_TO_MOVE_MASK;
    }

    pub fn can_castle_kingside(&self, color: u8) -> bool {
        self.value & (BLACK_KINGSIDE_CASTLE_MASK << (color * 2)) != 0
    }

    pub fn can_castle_queenside(&self, color: u8) -> bool {
        (self.value & (BLACK_QUEENSIDE_CASTLE_MASK) << (color * 2)) != 0
    }

    pub fn set_castling_rights(&mut self, castling_rights: u32) {
        self.value = (self.value & !CASTLING_RIGHTS_MASK) | (castling_rights << 1);
    }

    pub fn get_castling_rights(&self) -> u32 {
        (self.value & CASTLING_RIGHTS_MASK) >> 1
    }

    pub fn disable_all_castling_rights(&mut self, color: u8) {
        if color == WHITE {
            self.value &= !WHITE_CASTLING_RIGHTS_MASK;
        } else {
            self.value &= !BLACK_CASTLING_RIGHTS_MASK;
        }
    }

    pub fn disable_kingside_castling_rights(&mut self, color: u8) {
        if color == WHITE {
            self.value &= !WHITE_KINGSIDE_CASTLE_MASK;
        } else {
            self.value &= !BLACK_KINGSIDE_CASTLE_MASK;
        }
    }

    pub fn disable_queenside_castling_rights(&mut self, color: u8) {
        if color == WHITE {
            self.value &= !WHITE_QUEENSIDE_CASTLE_MASK;
        } else {
            self.value &= !BLACK_QUEENSIDE_CASTLE_MASK;
        }
    }

    pub fn set_castling_rights_for(&mut self, color: u8, side: u8) {
        let mask = match (color, side) {
            (WHITE, KING) => WHITE_KINGSIDE_CASTLE_MASK,
            (WHITE, QUEEN) => WHITE_QUEENSIDE_CASTLE_MASK,
            (BLACK, KING) => BLACK_KINGSIDE_CASTLE_MASK,
            (BLACK, QUEEN) => BLACK_QUEENSIDE_CASTLE_MASK,
            _ => panic!("Incalid value supplied to set castling for"),
        };
        self.value |= mask;
    }

    pub fn get_en_passant_file(&self) -> u32 {
        (self.value & EN_PASSANT_FILE_MASK) >> 5
    }

    pub fn set_en_passant_file(&mut self, file: u32) {
        self.value = (self.value & !EN_PASSANT_FILE_MASK) | (file << 5);
    }

    pub fn clear_en_passant_file(&mut self) {
        self.value &= !EN_PASSANT_FILE_MASK;
    }

    pub fn get_halfmove_clock(&self) -> u32 {
        (self.value & HALFMOVE_CLOCK_MASK) >> 9
    }

    pub fn set_halfmove_clock(&mut self, halfmove_clock: u32) {
        self.value = (self.value & !HALFMOVE_CLOCK_MASK) | (halfmove_clock << 9);
    }

    pub fn increment_halfmove_clock(&mut self) {
        self.value += 1 << 9;
    }

    pub fn clear_halfmove_clock(&mut self) {
        self.value &= !HALFMOVE_CLOCK_MASK;
    }

    pub fn set_move_clock(&mut self, move_clock: u32) {
        self.value = (self.value & !MOVE_CLOCK_MASK) | (move_clock << 17);
    }

    pub fn increase_move_clock(&mut self) {
        self.value += 1 << 17;
    }

    pub fn get_move_clock(&self) -> u32 {
        (self.value & MOVE_CLOCK_MASK) >> 17
    }

    pub fn get_captured_piece(&self) -> u32 {
        (self.value & CAPTURED_PIECE_MASK) >> 29
    }

    pub fn set_captured_piece(&mut self, piece: u8) {
        self.value = (self.value & !CAPTURED_PIECE_MASK) | ((piece as u32) << 29);
    }
}

const SIDE_TO_MOVE_MASK: u32 = 1;

const BLACK_KINGSIDE_CASTLE_MASK: u32 = 0b10;
const BLACK_QUEENSIDE_CASTLE_MASK: u32 = 0b100;
const WHITE_KINGSIDE_CASTLE_MASK: u32 = 0b1000;
const WHITE_QUEENSIDE_CASTLE_MASK: u32 = 0b10000;
const WHITE_CASTLING_RIGHTS_MASK: u32 = 0b11000;
const BLACK_CASTLING_RIGHTS_MASK: u32 = 0b110;
const CASTLING_RIGHTS_MASK: u32 = 0b11110;

const EN_PASSANT_FILE_MASK: u32 = 0b1111 << 5;
const HALFMOVE_CLOCK_MASK: u32 = 0xFF << 9;
const MOVE_CLOCK_MASK: u32 = 0xFFF << 17;

const CAPTURED_PIECE_MASK: u32 = 0b111 << 29;
