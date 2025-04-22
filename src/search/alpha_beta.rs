use std::{clone, i32};

const INFINITY: i32 = 10_000_000;

use crate::{
    board::board::Board,
    evaluation::evaluate,
    moving::{
        move_generation::{generate_moves, MoveList},
        mv::Move,
    },
};

use super::transposition::{Entry, TTable};

pub struct Searcher {
    ttable: TTable,
    search_depth: i32,
    ttable_hits: i32,
    nodes_searched: i32,
    max_depth: i32,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher {
            ttable: TTable::new(),
            search_depth: 0,
            ttable_hits: 0,
            nodes_searched: 0,
            max_depth: 0,
        }
    }

    pub fn search_to_depth(&mut self, board: &mut Board, depth: i32) -> Move {
        self.search_depth = depth;
        self.ttable_hits = 0;
        self.nodes_searched = 0;
        self.max_depth = 0;
        let mut best_value = -INFINITY;
        let mut moves = MoveList::new();
        let mut alpha = -INFINITY;
        let mut best_move = moves.get_move(0).clone();

        generate_moves(&mut moves, board);
        for mv in moves.iter() {
            board.make_move(mv);
            self.nodes_searched += 1;
            let score = -self.nega_max(board, 1, -INFINITY, -alpha);
            board.unmake_move(mv);
            if score > best_value {
                best_move = mv.clone();
                best_value = score;
                alpha = score;
            }
        }
        println!(
            "info depth {} cp {} tthits {} nodes searched {} depth reached {}",
            depth, best_value, self.ttable_hits, self.nodes_searched, self.max_depth
        );
        best_move
    }

    pub fn nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if depth == self.search_depth {
            return self.quiesce_nega_max(board, depth, alpha, beta);
        }
        self.nodes_searched += 1;
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
            let score = self.score(board, depth, alpha, beta);
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

    fn score(&mut self, board: &mut Board, depth: i32, alpha: i32, beta: i32) -> i32 {
        let depth_left = self.search_depth - depth;
        let hash = board.get_hash();
        let tt_entry = self.ttable.probe(hash);
        match tt_entry {
            Some(entry) if entry.depth_left >= depth_left => {
                self.ttable_hits += 1;
                entry.score
            }
            _ => {
                let score = -self.nega_max(board, depth + 1, -beta, -alpha);
                let new_entry = Entry {
                    key: board.get_hash(),
                    depth_left,
                    score,
                };
                self.ttable.store(new_entry);
                score
            }
        }
    }

    pub fn quiesce_nega_max(
        &mut self,
        board: &mut Board,
        depth: i32,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        if depth > self.max_depth {
            self.max_depth = depth;
        }
        self.nodes_searched += 1;
        let mut moves = MoveList::new();
        generate_moves(&mut moves, board);
        if moves.get_count() == 0 {
            if board.is_check() {
                return -100_000;
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
            if !mv.is_capture() {
                continue;
            }
            board.make_move(mv);
            let score = self.score_quiesce(board, depth, alpha, beta);
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

    fn score_quiesce(&mut self, board: &mut Board, depth: i32, alpha: i32, beta: i32) -> i32 {
        let depth_left = self.search_depth - depth;
        let hash = board.get_hash();
        let tt_entry = self.ttable.probe(hash);
        match tt_entry {
            Some(entry) if entry.depth_left >= depth_left => {
                self.ttable_hits += 1;
                entry.score
            }
            _ => {
                let score = -self.quiesce_nega_max(board, depth + 1, -beta, -alpha);
                let new_entry = Entry {
                    key: board.get_hash(),
                    depth_left,
                    score,
                };
                self.ttable.store(new_entry);
                score
            }
        }
    }
}
