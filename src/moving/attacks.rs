use crate::{bitboard_helpers::get_lsb, constants::{BISHOP, BLACK, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE}, moving::move_generation::MoveGenerator, position::board::Board};

impl MoveGenerator {
    pub fn attackers_to_exist(&self, board: &Board, k_sq: u64, occ: u64, color: usize) -> u64 {
        let pieces = board.get_pieces(color);
        let k_i = get_lsb(&k_sq);

        (self.get_pawn_attacks(k_i, color) & pieces.get_pawns()) |
        (self.get_knight_attacks(k_i) & pieces.get_knights()) |
        (self.get_rook_attacks(k_i, occ) & pieces.get_orthogonals()) |
        (self.get_bishop_attacks(k_i, occ) & pieces.get_diagonals())
    }

    pub fn attacks_to(&self, piece: usize, sq: usize, occ: u64, color: usize) -> u64 {
        match piece {
            PAWN => self.get_pawn_attacks(sq, color),
            KNIGHT => self.get_knight_attacks(sq),
            BISHOP => self.get_bishop_attacks(sq, occ),
            ROOK => self.get_rook_attacks(sq, occ),
            QUEEN => self.get_rook_attacks(sq, occ) | self.get_bishop_attacks(sq, occ),
            KING => self.get_king_attacks(sq),
            _ => panic!("Invalid value supplied to attacks_to")
        }
    }

    pub fn attacks_from(&self, piece: usize, sq: usize, occ: u64, color: usize) -> u64 {
        match piece {
            PAWN => self.pawn_attacks_from(sq, color),
            KNIGHT => self.get_knight_attacks(sq),
            BISHOP => self.get_bishop_attacks(sq, occ),
            ROOK => self.get_rook_attacks(sq, occ),
            QUEEN => self.get_rook_attacks(sq, occ) | self.get_bishop_attacks(sq, occ),
            KING => self.get_king_attacks(sq),
            _ => panic!("Invalid value supplied to attacks_from")
        }
    }

    pub fn pawn_attacks_from(&self, sq: usize, color: usize) -> u64 {
        if color == WHITE {
            self.get_pawn_attacks(sq, BLACK)
        } else {
            self.get_pawn_attacks(sq, WHITE)
        }
    }
}
