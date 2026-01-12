use std::collections::HashSet;
use crate::{moving::move_generation::generate_moves, position::board::Board};

// returns (false_positives, false_negatives)
pub fn test_transpositions(board: &mut Board, target_depth: i32) -> (u64, u64) {
    let mut false_positives = 0u64;
    let mut false_negatives= 0u64;
    let mut hashes: HashSet<u64> = HashSet::new();
    let mut tt_fens: HashSet<String> = HashSet::new();
    tt_test_helper(board, &mut false_positives, &mut false_negatives, &mut tt_fens, &mut hashes, target_depth);
    (false_positives, false_negatives)
}

fn tt_test_helper(board: &mut Board, false_positives: &mut u64, false_negatives: &mut u64, fens: &mut HashSet<String>, hashes: &mut HashSet<u64>, depth_left: i32) {
    if depth_left == 0 {
        let fen = board.to_fen_no_clocks();

        let in_fens = fens.contains(&fen);
        let in_hashes = hashes.contains(&board.get_hash());
        if in_fens && !in_hashes {
            *false_negatives += 1;
        }
        if !in_fens && in_hashes {
            *false_positives += 1;
        }
        fens.insert(fen);
        hashes.insert(board.get_hash());
        return;
    }

    let moves = generate_moves(board);
    for mv in moves.iter() {
        board.make_move(mv);
        tt_test_helper(board, false_positives, false_negatives, fens, hashes, depth_left - 1);
        board.unmake_move(mv);
    }
}

