use crate::constants::*;

pub struct PieceSet {
    pieces: [u64; 6],
    color: usize,
}

impl PieceSet {
    pub fn new(color: usize) -> Self {
        PieceSet {
            pieces: [0; 6],
            color,
        }
    }

    #[inline]
    pub fn get_all(&self) -> u64 {
        self.pieces.iter().fold(0, |acc, p| acc | p )
    }

    #[inline]
    pub fn get_queens(&self) -> u64 {
        self.pieces[ROOK] & self.pieces[BISHOP]
    }

    pub fn get_piece_at(&self, field: usize) -> usize {
        let pointer: u64 = 1 << field;
        let queens = self.get_queens();
        if pointer & queens != 0 {
            return QUEEN
        }
        for p in PAWN..=KING {
            if pointer & self.pieces[p] != 0 {
                return p;
            }
        }
        NONE
    }

    #[inline]
    pub fn make_queenside_castle(&mut self) {
        if self.color == WHITE {
            self.pieces[KING] = 0b100;
            self.pieces[ROOK] &= !1;
            self.pieces[ROOK] |= 0b1000;
        } else {
            self.pieces[KING] = 0x400000000000000;
            self.pieces[ROOK] &= !0x100000000000000;
            self.pieces[ROOK] |= 0x800000000000000;
        }
    }

    #[inline]
    pub fn unmake_queenside_castle(&mut self) {
        if self.color == WHITE {
            self.pieces[KING] = 0x10;
            self.pieces[ROOK] &= !0b1000;
            self.pieces[ROOK] |= 1;
        } else {
            self.pieces[KING] = 0x1000000000000000;
            self.pieces[ROOK] &= !0x800000000000000;
            self.pieces[ROOK] |= 0x100000000000000;
        }
    }

    #[inline]
    pub fn make_kingside_castle(&mut self) {
        if self.color == WHITE {
            self.pieces[KING] = 0x40;
            self.pieces[ROOK] &= !0x80;
            self.pieces[ROOK] |= 0x20;
        } else {
            self.pieces[KING] = 0x4000000000000000;
            self.pieces[ROOK] &= !0x8000000000000000;
            self.pieces[ROOK] |= 0x2000000000000000;
        }
    }

    #[inline]
    pub fn unmake_kingside_castle(&mut self) {
        if self.color == WHITE {
            self.pieces[KING] = 0x10;
            self.pieces[ROOK] &= !0x20;
            self.pieces[ROOK] |= 0x80;
        } else {
            self.pieces[KING] = 0x1000000000000000;
            self.pieces[ROOK] &= !0x2000000000000000;
            self.pieces[ROOK] |= 0x8000000000000000;
        }
    }

    #[inline]
    pub fn move_piece(&mut self, start: usize, target: usize) {
        let start_mask: u64 = 1 << start;
        let target_mask: u64 = 1 << target;
        if start_mask & self.pieces[PAWN] != 0 {
            self.pieces[PAWN] &= !start_mask;
            self.pieces[PAWN] |= target_mask;
            return;
        }
        if start_mask & self.pieces[ROOK] != 0 {
            self.pieces[ROOK] &= !start_mask;
            self.pieces[ROOK] |= target_mask;
            //no return because of queen
        }
        if start_mask & self.pieces[BISHOP] != 0 {
            self.pieces[BISHOP] &= !start_mask;
            self.pieces[BISHOP] |= target_mask;
            return;
        }
        if start_mask & self.pieces[KNIGHT] != 0 {
            self.pieces[KNIGHT] &= !start_mask;
            self.pieces[KNIGHT] |= target_mask;
            return;
        }
        if start_mask & self.pieces[KING] != 0 {
            self.pieces[KING] = target_mask;
        }
    }

    #[inline]
    pub fn take(&mut self, field: usize) {
        let mask: u64 = !(1 << field);
        self.pieces[PAWN] &= mask;
        self.pieces[BISHOP] &= mask;
        self.pieces[ROOK] &= mask;
        self.pieces[KNIGHT] &= mask;
    }

    #[inline]
    pub fn add_piece(&mut self, field: usize, piece_type: usize) {
        let mask: u64 = 1 << field;
        if piece_type == QUEEN {
            self.pieces[ROOK] |= mask;
            self.pieces[BISHOP] |= mask;
        } else {
            self.pieces[piece_type] |= mask;
        }
    }

    #[inline]
    pub fn get_pawns(&self) -> u64 {
        self.pieces[PAWN]
    }

    pub fn set_pawns(&mut self, pawns: u64) {
        self.pieces[PAWN] = pawns;
    }

    #[inline]
    pub fn get_orthogonals(&self) -> u64 {
        self.pieces[ROOK]
    }

    #[inline]
    pub fn get_rooks(&self) -> u64 {
        self.pieces[ROOK] & !self.pieces[BISHOP]
    }

    #[inline]
    pub fn get_bishops(&self) -> u64 {
        self.pieces[BISHOP] & !self.pieces[ROOK]
    }

    pub fn set_orthognals(&mut self, orthogonals: u64) {
        self.pieces[ROOK] = orthogonals;
    }

    #[inline]
    pub fn get_knights(&self) -> u64 {
        self.pieces[KNIGHT]
    }

    pub fn set_knights(&mut self, knights: u64) {
        self.pieces[KNIGHT] = knights;
    }

    #[inline]
    pub fn get_diagonals(&self) -> u64 {
        self.pieces[BISHOP]
    }

    pub fn set_diagonals(&mut self, diagonals: u64) {
        self.pieces[BISHOP] = diagonals;
    }

    #[inline]
    pub fn get_king(&self) -> u64 {
        self.pieces[KING]
    }

    pub fn set_king(&mut self, king: u64) {
        self.pieces[KING] = king;
    }

    pub fn get_color(&self) -> usize {
        self.color
    }

    pub fn iter(&self) -> PieceSetIter {
        PieceSetIter::new(&self.pieces)
    }

    pub fn iter_w_vals(&self) -> PieceSetValuesIter {
        PieceSetValuesIter::new(&self.pieces)
    }

}

pub struct PieceSetIter<'a> {
    pieces: &'a [u64; 6],
    idx: usize,
}

impl<'a> PieceSetIter<'a> {
    fn new(pieces: &'a [u64; 6]) -> Self {
        PieceSetIter {pieces, idx: 0}
    } 
}

impl<'a> Iterator for PieceSetIter<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.idx >= 6 {
            None
        } else {
            let res = self.pieces[self.idx];
            self.idx += 1;
            Some(res)
        }
    }
}

pub struct PieceSetValuesIter<'a> {
    pieces: &'a [u64; 6],
    values: [usize; 6],
    idx: usize
}

impl<'a> PieceSetValuesIter<'a> {
    fn new(pieces: &'a [u64; 6]) -> Self {
        PieceSetValuesIter { pieces, values: [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING], idx: 0 }
    }
}

impl<'a> Iterator for PieceSetValuesIter<'a> {
    type Item = (u64, usize);

    fn next(&mut self) -> Option<(u64, usize)> {
        if self.idx >= 6 {
            None
        } else {
            let pieces = self.pieces[self.idx];
            let value = self.values[self.idx];
            self.idx += 1;
            Some((pieces, value))
        }
    }
}
