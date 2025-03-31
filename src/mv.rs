use std::clone;

use crate::{board::Board, constants::*, piece_set::PieceSet};

pub struct Move {
    value: u16,
}

impl Move {
    pub fn new(start: u16, target: u16, code: u16) -> Move {
        Move {
            value: start << 10 | target << 4 | code,
        }
    }

    pub fn get_start_bb(&self) -> u64 {
        1 << self.get_start_field()
    }

    pub fn get_target_bb(&self) -> u64 {
        1 << self.get_target_field()
    }

    pub fn new_null_mv() -> Move {
        Move { value: 0 }
    }

    pub fn new_quiet(start: u16, target: u16) -> Move {
        Self::new(start, target, 0)
    }

    pub fn new_double_pawn_push(start: u16, target: u16) -> Move {
        Self::new(start, target, 1)
    }

    pub fn new_kingside_castle(start: u16, target: u16) -> Move {
        Self::new(start, target, 2)
    }

    pub fn new_queenside_castle(start: u16, target: u16) -> Move {
        Self::new(start, target, 3)
    }

    pub fn new_capture(start: u16, target: u16) -> Move {
        Self::new(start, target, 4)
    }

    pub fn new_en_passant(start: u16, target: u16) -> Move {
        Self::new(start, target, 5)
    }

    pub fn new_promotion(start: u16, target: u16, piece: u8) -> Move {
        let code = 8 + piece - 2;
        Self::new(start, target, code as u16)
    }

    pub fn new_promotion_capture(start: u16, target: u16, piece: u8) -> Move {
        let code = 12 + piece - 2;
        Self::new(start, target, code as u16)
    }

    pub fn is_capture(&self) -> bool {
        self.value & (1 << 2) != 0
    }

    pub fn is_en_passant(&self) -> bool {
        self.get_move_code() == 0b0101
    }

    pub fn is_promotion(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    //this DOES NOT check wether move was a promotion
    pub fn get_promotion_piece(&self) -> u8 {
        ((self.value & 0b11) + 2) as u8
    }

    pub fn is_double_pawn_move(&self) -> bool {
        self.value & CODE_MASK == 1
    }

    pub fn is_kingside_castle(&self) -> bool {
        self.value & CODE_MASK == 2
    }

    pub fn is_queenside_castle(&self) -> bool {
        self.value & CODE_MASK == 3
    }

    pub fn get_start_field(&self) -> u16 {
        (self.value & START_MASK) >> 10
    }

    pub fn get_target_field(&self) -> u16 {
        (self.value & TARGET_MASK) >> 4
    }

    pub fn get_move_code(&self) -> u16 {
        self.value & CODE_MASK
    }

    pub fn to_str(&self) -> String {
        let start_str = field_to_str(self.get_start_field());
        let target_str = field_to_str(self.get_target_field());
        format!("{}{}", start_str, target_str)
    }
}

fn field_to_str(field: u16) -> String {
    let row = (field / 8) as u32;
    let file = (field % 8) as u8;
    let file_char = (97 + file) as char;
    let row_char = char::from_digit(row + 1, 10).unwrap();
    let mut res = String::new();
    res.push(file_char);
    res.push(row_char);
    res
}

const CODE_MASK: u16 = 0xF;
const TARGET_MASK: u16 = 0x3F << 4;
const START_MASK: u16 = 0x3F << 10;
