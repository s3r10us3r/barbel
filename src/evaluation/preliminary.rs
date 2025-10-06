use crate::{bitboard_helpers::{get_lsb, pop_lsb}, constants::{BLACK, FILEA, FILEH, FILES, WHITE}, evaluation::{phase::get_phase_val, Evaluator}, moving::move_generation::MoveGenerator, position::board::Board};

pub struct PreEvalResult {
    pub attack_map: SimpAttackMap,
    pub half_open_files: [u64; 2],
    pub open_files: u64,
    pub phase: i32
}

impl Evaluator {
    pub fn run_pre_eval(&self, board: &Board) -> PreEvalResult {
        let colors = [WHITE, BLACK];
        let mut half_open_files_arr = [0u64, 0u64];

        for color in colors {
            let pawns = board.players[color].get_pawns();
            let mut half_open_files = 0u64;
            for file in FILES {
                if file & pawns == 0 {
                    half_open_files |= file;
                }
            }
            half_open_files_arr[color] = half_open_files;
        };

        let open_files = half_open_files_arr[BLACK] & half_open_files_arr[WHITE];
        half_open_files_arr[BLACK] &= !open_files;
        half_open_files_arr[WHITE] &= !open_files;

        let phase = get_phase_val(board);
        let attack_map = SimpAttackMap::new(board, &board.mg);
        PreEvalResult {attack_map, half_open_files: half_open_files_arr, open_files, phase}
    }
}

pub struct SimpAttackMap {
    pub attack_maps: [u64; 6],
}

impl SimpAttackMap {
    pub fn new(board: &Board, mg: &MoveGenerator) -> Self {
        let pieces = board.get_pieces(board.us);

        let pawn_attacks = if board.us == WHITE {
            let pawns = pieces.get_pawns();
            ((pawns & !FILEA) << 7) | ((pawns & !FILEH) << 9)
        } else {
            let pawns = pieces.get_pawns();
            ((pawns & !FILEA) >> 9) | ((pawns & !FILEH) >> 7)
        };

        let occ = board.get_occupancy();

        let knight_attacks = compute_piece_attacks(pieces.get_knights(), |sq| mg.get_knight_attacks(sq));
        let bishop_attacks = compute_piece_attacks(pieces.get_bishops(), |sq| mg.get_bishop_attacks(sq, occ));
        let rook_attacks = compute_piece_attacks(pieces.get_rooks(), |sq| mg.get_rook_attacks(sq, occ));
        let queen_attacks = compute_piece_attacks(pieces.get_queens(), |sq| mg.get_rook_attacks(sq, occ) | mg.get_bishop_attacks(sq, occ));
        let king_attacks = mg.get_king_attacks(get_lsb(&pieces.get_king()));

        SimpAttackMap {attack_maps: [pawn_attacks, knight_attacks, bishop_attacks, rook_attacks, queen_attacks, king_attacks]}
    }
}

fn compute_piece_attacks<F>(mut piece_bb: u64, op: F) -> u64 
where 
    F: Fn(usize) -> u64,
{
    let mut result = 0u64;
    while piece_bb != 0 {
        let lsb = pop_lsb(&mut piece_bb);
        result |= op(lsb);
    }
    result
}
