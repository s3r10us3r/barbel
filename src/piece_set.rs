use crate::constants::{BISHOP, KING, KNIGHT, NONE, PAWN, QUEEN, ROOK};

pub struct PieceSet {
    pawns: u64,
    orthogonals: u64,
    diagonals: u64,
    knights: u64,
    king: u64,
}

impl PieceSet {
    pub fn new() -> Self {
        PieceSet {
            pawns: 0,
            orthogonals: 0,
            diagonals: 0,
            knights: 0,
            king: 0,
        }
    }

    pub fn get_all(&self) -> u64 {
        self.pawns | self.orthogonals | self.diagonals | self.knights | self.king
    }

    pub fn get_queens(&self) -> u64 {
        self.diagonals & self.orthogonals
    }

    pub fn get_piece_at(&self, field: i32) -> u8 {
        let pointer: u64 = 1 << field;
        if pointer & self.pawns != 0 {
            return PAWN;
        }
        let is_orthogonal = pointer & self.orthogonals != 0;
        let is_diagonal = pointer & self.diagonals != 0;
        if is_orthogonal && is_diagonal {
            return QUEEN;
        }
        if is_orthogonal {
            return ROOK;
        }
        if is_diagonal {
            return BISHOP;
        }
        if pointer & self.knights != 0 {
            return KNIGHT;
        }
        if pointer & self.king != 0 {
            return KING;
        }
        NONE
    }

    pub fn move_piece(&mut self, start: i32, target: i32) {
        let start_mask: u64 = 1 << start;
        let target_mask: u64 = 1 << target;
        if start_mask | self.pawns != 0 {
            self.pawns &= !start_mask;
            self.pawns |= target_mask;
            return;
        }
        if start_mask & self.orthogonals != 0 {
            self.orthogonals &= !start_mask;
            self.orthogonals |= target_mask;
            //no return because of queen
        }
        if start_mask & self.diagonals != 0 {
            self.diagonals &= !start_mask;
            self.diagonals |= target_mask;
            return;
        }
        if start_mask & self.knights != 0 {
            self.knights &= !start_mask;
            self.knights |= start_mask;
            return;
        }
        if start_mask | self.king != 0 {
            self.king = target_mask;
        }
    }

    pub fn take(&mut self, field: i32) {
        let mask: u64 = !(1 << field);
        self.pawns &= mask;
        self.diagonals &= mask;
        self.orthogonals &= mask;
        self.knights &= mask;
    }

    pub fn add_piece(&mut self, field: i32, piece_type: u8) {
        let mask: u64 = 1 << field;
        match piece_type {
            PAWN => self.pawns |= mask,
            ROOK => self.orthogonals |= mask,
            BISHOP => self.diagonals |= mask,
            KNIGHT => self.knights |= mask,
            KING => self.king |= mask,
            QUEEN => {
                self.orthogonals |= mask;
                self.diagonals |= mask;
            }
            _ => panic!("Invalid piece type {piece_type} in PieceSet.add_piece"),
        }
    }

    pub fn get_pawns(&self) -> u64 {
        self.pawns
    }

    pub fn set_pawns(&self) -> u64 {
        self.pawns
    }

    pub fn get_orthogonals(&self) -> u64 {
        self.orthogonals
    }

    pub fn set_orthognals(&mut self, orthogonals: u64) {
        self.orthogonals = orthogonals;
    }

    pub fn get_knights(&self) -> u64 {
        self.knights
    }

    pub fn set_knights(&mut self, knights: u64) {
        self.knights = knights;
    }

    pub fn get_diagonals(&self) -> u64 {
        self.diagonals
    }

    pub fn set_diagonals(&mut self, diagonals: u64) {
        self.diagonals = diagonals;
    }

    pub fn get_king(&self) -> u64 {
        self.king
    }

    pub fn set_king(&mut self, king: u64) {
        self.king = king;
    }
}
