use alpha_beta::Searcher;

use crate::board::board::Board;

mod alpha_beta;

pub fn search_to_depth(board: &mut Board, depth: i32) {
    let searcher = Searcher::new();
    let mv = searcher.search_to_depth(board, depth);
    println!("bestmove {}", mv.to_str());
}
