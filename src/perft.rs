use std::time::Instant;

use crate::{
    board::Board,
    fen_parsing::{parse_fen, FenError},
    move_generation::{generate_moves, MoveList},
};

pub struct PerftResult {
    pub time: f32,
    pub result: usize,
}

pub fn make_perft(fen: &str, depth: i32) -> Result<PerftResult, FenError> {
    let mut board = parse_fen(fen)?;
    let start = Instant::now();
    let result = test_perft(&mut board, depth);
    let time = start.elapsed().as_secs_f32();
    Ok(PerftResult { time, result })
}

fn test_perft(board: &mut Board, depth_left: i32) -> usize {
    let mut new_move_list = MoveList::new();
    generate_moves(&mut new_move_list, board);
    if depth_left <= 1 {
        new_move_list.get_count()
    } else {
        let mut i = 0;
        let mut result = 0;
        while i < new_move_list.get_count() {
            let mv = &new_move_list[i];
            board.make_move(mv);
            result += test_perft(board, depth_left - 1);
            board.unmake_move(mv);
            i += 1;
        }
        result
    }
}
