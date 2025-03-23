pub const NONE: u8 = 0;
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;

pub const WHITE: u8 = 1;
pub const BLACK: u8 = 0;

pub const VERTICAL_SHIFT: u8 = 8;
pub const HORIZONTAL_SHIFT: u8 = 1;

pub const RANK1: u64 = 0b11111111;
pub const RANK2: u64 = RANK1 << VERTICAL_SHIFT;
pub const RANK3: u64 = RANK2 << VERTICAL_SHIFT;
pub const RANK4: u64 = RANK3 << VERTICAL_SHIFT;
pub const RANK5: u64 = RANK4 << VERTICAL_SHIFT;
pub const RANK6: u64 = RANK5 << VERTICAL_SHIFT;
pub const RANK7: u64 = RANK6 << VERTICAL_SHIFT;
pub const RANK8: u64 = RANK7 << VERTICAL_SHIFT;

pub const FILEA: u64 = 0x101010101010101;
pub const FILEB: u64 = FILEA << HORIZONTAL_SHIFT;
pub const FILEC: u64 = FILEB << HORIZONTAL_SHIFT;
pub const FILED: u64 = FILEC << HORIZONTAL_SHIFT;
pub const FILEE: u64 = FILED << HORIZONTAL_SHIFT;
pub const FILEF: u64 = FILEE << HORIZONTAL_SHIFT;
pub const FILEG: u64 = FILEF << HORIZONTAL_SHIFT;
pub const FILEH: u64 = FILEG << HORIZONTAL_SHIFT;
