use crate::constants::{BLACK, FILES, WHITE};
use crate::evaluation::phase::get_phase_val;
use crate::evaluation::Evaluator;
use crate::position::board::Board;

pub struct PreEvalResult {
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
        PreEvalResult {half_open_files: half_open_files_arr, open_files, phase}
    }
}

