use std::{f32, i32};

const INFINITY: i32 = 10_000_000;

use crate::{
    board::board::Board,
    evaluation::evaluate,
    moving::{
        move_generation::{generate_moves, MoveList},
        mv::Move,
    },
};

pub struct Searcher {
    pv_nodes: [Move; 100],
    pv_nodes_size: usize,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher {
            pv_nodes: std::array::from_fn(|_| Move::new_null_mv()),
            pv_nodes_size: 0,
        }
    }

    pub fn search_to_depth(&self, board: &mut Board, depth: i32) -> Move {
        let mut best_value = -INFINITY;
        let mut moves = MoveList::new();
        let mut alpha = -INFINITY;
        let mut best_move = Move::new_null_mv();

        generate_moves(&mut moves, board);
        for mv in moves.iter() {
            board.make_move(mv);
            let score = -nega_max(board, depth - 1, -INFINITY, -alpha);
            board.unmake_move(mv);
            if score > best_value {
                best_move = mv.clone();
                best_value = score;
                if score > alpha {
                    alpha = score;
                }
            }
        }
        best_move
    }
}

pub fn nega_max(board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return quiesce_nega_max(board, depth, alpha, beta);
    }
    let mut moves = MoveList::new();
    generate_moves(&mut moves, board);
    if moves.get_count() == 0 {
        if board.is_check() {
            return -INFINITY;
        } else {
            return 0;
        }
    }
    let mut best_value = -INFINITY;
    for mv in moves.iter() {
        board.make_move(mv);
        let score = -nega_max(board, depth - 1, -beta, -alpha);
        board.unmake_move(mv);
        if score > best_value {
            best_value = score;
            if score > alpha {
                alpha = score;
            }
        }
        if score >= beta {
            return best_value;
        }
    }
    best_value
}

pub fn quiesce_nega_max(board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    let mut moves = MoveList::new();
    generate_moves(&mut moves, board);
    if moves.get_count() == 0 {
        if board.is_check() {
            return -INFINITY;
        } else {
            return 0;
        }
    }
    let mut best_value = evaluate(board);
    if best_value >= beta {
        return best_value;
    }
    if alpha < best_value {
        alpha = best_value
    }
    for mv in moves.iter() {
        if !mv.is_non_quiet() {
            continue;
        }
        board.make_move(mv);
        let score = -quiesce_nega_max(board, depth - 1, -beta, -alpha);
        board.unmake_move(mv);
        if score > best_value {
            best_value = score;
            if score > alpha {
                alpha = score;
            }
        }
        if score >= beta {
            return best_value;
        }
    }
    best_value
}
