use crate::{bitboard_helpers::*, board::board::Board, constants::*, lookups::*};

impl Board {
    pub fn attackers_to_exist(&self, square: u64, occupancy: u64, color: usize) -> u64 {
        let pieces = self.get_pieces(color);
        let mut attackers: u64 = 0;
        let square_i = square.trailing_zeros() as usize;

        attackers |= PAWN_ATTACKS_TO[color][square_i] & pieces.get_pawns();
        attackers |= KNIGHT_LOOKUP[square_i] & pieces.get_knights();
        attackers |= orthogonal_attacks_to(square, occupancy) & pieces.get_orthogonals();
        attackers |= diagonal_attacks_to(square, occupancy) & pieces.get_diagonals();
        attackers |= KING_LOOKUP[square_i] & pieces.get_king();

        attackers
    }
}

pub fn attacks_to(piece: u8, square: u64, occupancy: u64, color: usize) -> u64 {
    match piece {
        PAWN => PAWN_ATTACKS_TO[color][square.trailing_zeros() as usize],
        KNIGHT => KNIGHT_LOOKUP[square.trailing_zeros() as usize],
        BISHOP => diagonal_attacks_to(square, occupancy),
        ROOK => orthogonal_attacks_to(square, occupancy),
        QUEEN => diagonal_attacks_to(square, occupancy) | orthogonal_attacks_to(square, occupancy),
        KING => KING_LOOKUP[square.trailing_zeros() as usize],
        _ => panic!("Invalid value supplied to attacks_to"),
    }
}

pub fn attacks_from(piece: u8, square: u64, occupancy: u64, color: usize) -> u64 {
    match piece {
        PAWN => pawn_attacks_from(square, color),
        KNIGHT => KNIGHT_LOOKUP[square.trailing_zeros() as usize],
        BISHOP => diagonal_attacks_from(square, occupancy),
        ROOK => orthogonal_attacks_from(square, occupancy),
        QUEEN => {
            diagonal_attacks_from(square, occupancy) | orthogonal_attacks_from(square, occupancy)
        }
        KING => KING_LOOKUP[square.trailing_zeros() as usize],
        _ => panic!("Invalid value supplied to attacks_from"),
    }
}

pub fn pawn_attacks_from(square: u64, color: usize) -> u64 {
    if color == WHITE {
        ((square & !FILEA) << 7) | ((square & !FILEH) << 9)
    } else {
        ((square & !FILEA) >> 9) | ((square & !FILEH) >> 7)
    }
}

pub fn orthogonal_attacks_to(square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    map_positive_ray_attacks(square_i, occupancy, N8)
        | map_positive_ray_attacks(square_i, occupancy, E8)
        | map_negative_ray_attacks(square_i, occupancy, S8)
        | map_negative_ray_attacks(square_i, occupancy, W8)
}

pub fn orthogonal_attacks_from(square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    map_positive_ray_attacks(square_i, occupancy, N8)
        | map_positive_ray_attacks(square_i, occupancy, E8)
        | map_negative_ray_attacks(square_i, occupancy, S8)
        | map_negative_ray_attacks(square_i, occupancy, W8)
}

pub fn diagonal_attacks_to(square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    map_positive_ray_attacks(square_i, occupancy, NE8)
        | map_positive_ray_attacks(square_i, occupancy, NW8)
        | map_negative_ray_attacks(square_i, occupancy, SE8)
        | map_negative_ray_attacks(square_i, occupancy, SW8)
}

pub fn diagonal_attacks_from(square: u64, occupancy: u64) -> u64 {
    let square_i = square.trailing_zeros() as usize;
    map_positive_ray_attacks(square_i, occupancy, NE8)
        | map_positive_ray_attacks(square_i, occupancy, NW8)
        | map_negative_ray_attacks(square_i, occupancy, SE8)
        | map_negative_ray_attacks(square_i, occupancy, SW8)
}

pub fn map_positive_ray_attacks(index: usize, occupied: u64, dir: usize) -> u64 {
    let attacks = RAY_LOOKUP[dir][index];
    let blocker = attacks & occupied;
    if blocker != 0 {
        let blocker_i = get_lsb(&blocker);
        attacks ^ RAY_LOOKUP[dir][blocker_i]
    } else {
        attacks
    }
}

pub fn map_negative_ray_attacks(index: usize, occupied: u64, dir: usize) -> u64 {
    let attacks = RAY_LOOKUP[dir][index];
    let blocker = attacks & occupied;
    if blocker != 0 {
        let index = get_msb(&blocker);
        attacks ^ RAY_LOOKUP[dir][index]
    } else {
        attacks
    }
}
