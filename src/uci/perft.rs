use std::time::Instant;

use crate::{
    lookups::simple_lookups::MoveGenerator, moving::{move_generation::generate_moves, mv::Move}, position::board::Board
};

pub struct PerftResult {
    pub time: f32,
    pub result: usize,
}

pub fn make_perft(depth: i32, board: &mut Board) -> PerftResult {
    let move_gen = MoveGenerator::new();

    let start = Instant::now();
    let mut result = 0;
    let mv_list = move_gen.generate_moves(board);
    let count = mv_list.get_count();
    for i in 0..count {
        let mv = &mv_list[i];
        board.make_move(mv);
        let mv_result = test_perft(board, depth - 1, &move_gen);
        result += mv_result;
        board.unmake_move(mv);
        println!("{}: {}", mv.to_str(), mv_result);
    }
    let time = start.elapsed().as_secs_f32();
    PerftResult { time, result }
}

fn test_perft(board: &mut Board, depth_left: i32, move_gen: &MoveGenerator) -> usize {
    if depth_left == 0 {
        return 1;
    }
    let new_move_list = move_gen.generate_moves(board);
    if depth_left <= 1 {
        return new_move_list.get_count();
    }
    let mut i = 0;
    let mut result = 0;
    while i < new_move_list.get_count() {
        let mv = &new_move_list[i];
        board.make_move(mv);
        let mv_res = test_perft(board, depth_left - 1, move_gen);
        result += mv_res;
        board.unmake_move(mv);
        i += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use crate::fen_parsing::parse_fen::parse_fen;

    use super::make_perft;

    #[test]
    fn should_return_correct_perft_result_for_startpos_depth_5() {
        should_return_correct_perft_result(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            4_865_609,
            5,
        );
    }

    #[test]
    fn should_return_correct_perft_result_for_kiwipete_depth_4() {
        should_return_correct_perft_result(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            4_085_603,
            4,
        );
    }

    #[test]
    fn should_return_correct_perft_result_for_pos_3_depth_5() {
        should_return_correct_perft_result("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 674624, 5);
    }

    #[test]
    fn should_return_correct_perft_result_for_pos_4_depth_4() {
        should_return_correct_perft_result(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            422_333,
            4,
        );
    }

    #[test]
    fn should_return_correct_perft_result_for_pos_5_depth_4() {
        should_return_correct_perft_result(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            2_103_487,
            4,
        );
    }

    #[test]
    fn should_return_correct_perft_result_for_pos_6_depth_4() {
        should_return_correct_perft_result(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            3_894_594,
            4,
        );
    }

    fn should_return_correct_perft_result(fen: &str, expected: usize, depth: i32) {
        let mut board = parse_fen(fen).unwrap();
        let result = make_perft(depth, &mut board);
        assert_eq!(expected, result.result);
    }
}
