use std::time::Instant;

use crate::{fen_parsing::parse_fen::parse_fen, search::alpha_beta::Searcher};

const POSITIONS: [&str; 5] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
    "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
    "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"
];

const DEPTHS: [i32; 5] = [9, 7, 8, 8, 8];

pub fn make_comp_tests() {
    let mut flat_sum = 0f32;
    let mut iter_sum = 0f32;
    let mut cnt = 0;

    for (position, depth) in POSITIONS.iter().zip(DEPTHS.iter()) {
        cnt += 1;
        println!("Position {cnt}");
        let mut board = parse_fen(position).unwrap();
        
        let mut searcher = Searcher::new();
        let start = Instant::now();
        searcher.search_flat(&mut board, *depth);
        let flat_time = start.elapsed().as_secs_f32();
        let node_cnt = searcher.get_nodes_searched();
        println!("Fixed depth time: {flat_time} node count: {node_cnt}");


        let mut searcher = Searcher::new();
        let start = Instant::now();
        searcher.search_to_depth(&mut board, *depth);
        let iter_time = start.elapsed().as_secs_f32();
        let node_cnt = searcher.get_nodes_searched();
        println!("Iterative deepening time: {iter_time} node count: {node_cnt}\n");

        flat_sum += flat_time;
        iter_sum += iter_time;
    }

    let iter_avg = iter_sum / 5.;
    let flat_avg = flat_sum / 5.;

    println!("Fixed depth time  sum: {flat_sum}  avg: {flat_avg}");
    println!("Iterative deepening time   sum: {iter_sum}  avg: {iter_avg}");
}
