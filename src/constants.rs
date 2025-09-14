pub const PAWN: usize = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;
pub const NONE: usize = 6;

pub const WHITE: usize = 1;
pub const BLACK: usize = 0;

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

pub const RANKS: [u64; 8] = [RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8];

pub const FILEA: u64 = 0x101010101010101;
pub const FILEB: u64 = FILEA << HORIZONTAL_SHIFT;
pub const FILEC: u64 = FILEB << HORIZONTAL_SHIFT;
pub const FILED: u64 = FILEC << HORIZONTAL_SHIFT;
pub const FILEE: u64 = FILED << HORIZONTAL_SHIFT;
pub const FILEF: u64 = FILEE << HORIZONTAL_SHIFT;
pub const FILEG: u64 = FILEF << HORIZONTAL_SHIFT;
pub const FILEH: u64 = FILEG << HORIZONTAL_SHIFT;

pub const FILES: [u64; 8] = [FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH];

pub const KINGSIDE_CASTLE_MASK: u64 = 0x60;
pub const QUEENSIDE_CATLE_MASK: u64 = 0xE;
