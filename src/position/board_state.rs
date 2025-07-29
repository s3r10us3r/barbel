use crate::constants::{BLACK, KING, QUEEN, WHITE};

#[derive(Clone, Default)]
pub struct BoardState {
    value: u32,
}


impl BoardState {
    pub fn new() -> Self {
        BoardState { value: 0 }
    }

    #[inline]
    pub fn can_castle_kingside(&self, color: usize) -> bool {
        self.value & KINGSIDE_CASTLE_MASKS[color] != 0
    }

    #[inline]
    pub fn can_castle_queenside(&self, color: usize) -> bool {
        self.value & QUEENSIDE_CASTLE_MASKS[color] != 0
    }

    pub fn set_castling_rights(&mut self, castling_rights: u32) {
        self.value = (self.value & !CASTLING_RIGHTS_MASK) | (castling_rights << 1);
    }

    #[inline]
    pub fn get_castling_rights(&self) -> u32 {
        (self.value & CASTLING_RIGHTS_MASK) >> 1
    }

    #[inline]
    pub fn disable_all_castling_rights(&mut self, color: usize) {
        self.value &= !CASTLE_MASKS[color];
    }

    #[inline]
    pub fn disable_kingside_castling_rights(&mut self, color: usize) {
        self.value &= !KINGSIDE_CASTLE_MASKS[color];
    }

    #[inline]
    pub fn disable_queenside_castling_rights(&mut self, color: usize) {
        self.value &= !QUEENSIDE_CASTLE_MASKS[color];
    }

    pub fn set_castling_rights_for(&mut self, color: usize, side: usize) {
        let mask = match (color, side) {
            (WHITE, KING) => WHITE_KINGSIDE_CASTLE_MASK,
            (WHITE, QUEEN) => WHITE_QUEENSIDE_CASTLE_MASK,
            (BLACK, KING) => BLACK_KINGSIDE_CASTLE_MASK,
            (BLACK, QUEEN) => BLACK_QUEENSIDE_CASTLE_MASK,
            _ => panic!("Invalid value supplied to set castling for"),
        };
        self.value |= mask;
    }

    #[inline]
    pub fn get_en_passant_file(&self) -> usize {
        ((self.value & EN_PASSANT_FILE_MASK) >> 5) as usize
    }

    pub fn set_en_passant_file(&mut self, file: usize) {
        let file = file as u32;
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

    #[inline]
    pub fn get_captured_piece(&self) -> u32 {
        (self.value & CAPTURED_PIECE_MASK) >> 29
    }

    pub fn set_captured_piece(&mut self, piece: usize) {
        self.value = (self.value & !CAPTURED_PIECE_MASK) | ((piece as u32) << 29);
    }
}

const QUEENSIDE_CASTLE_MASKS: [u32; 2] = [BLACK_QUEENSIDE_CASTLE_MASK, WHITE_QUEENSIDE_CASTLE_MASK];
const KINGSIDE_CASTLE_MASKS: [u32; 2] = [BLACK_KINGSIDE_CASTLE_MASK, WHITE_KINGSIDE_CASTLE_MASK];
const CASTLE_MASKS: [u32; 2] = [BLACK_CASTLING_RIGHTS_MASK, WHITE_CASTLING_RIGHTS_MASK];
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
