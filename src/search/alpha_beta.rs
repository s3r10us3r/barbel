use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use crate::evaluation::Evaluator;
use crate::search::transposition::TTEntryType;
use crate::{
    position::board::Board, constants::WHITE, 
    moving::{move_list::MoveList, mv::Move}
};

use super::{
    pv_table::PvTable,
    transposition::{Entry, TTable},
};


const INFINITY: i32 = 10_000_000;
const MATE: i32 = 1_000_000;


pub struct SearchResult {
    pub mv: Move,
    pub nodes_searched: i32,
    pub depth_reached: i32,
    pub ttable_hits: i32,
}


pub struct Searcher {
    ttable: TTable,
    pv_table: PvTable,
    evaluator: Evaluator,
    search_depth: i32,
    ttable_hits: i32,
    nodes_searched: i32,
    generation: i32,
    stop: Arc<AtomicBool>,
}

impl Default for Searcher {
    fn default() -> Self {
        Searcher {
            ttable: TTable::new(),
            pv_table: PvTable::new(2), //the size is arbitrary it gets overriden on new search
            evaluator: Evaluator::new(),
            search_depth: 0,
            ttable_hits: 0,
            nodes_searched: 0,
            generation: 0,
            stop: Arc::new(AtomicBool::new(false))
        }
    }
}

impl Searcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_to_depth(&mut self, board: &mut Board, depth: i32) -> SearchResult {
        self.nodes_searched = 0;
        self.pv_table = PvTable::new(depth as usize);
        for i in 1..(depth + 1) {
            self.make_search(board, i);
        }
        let mv = self.pv_table.get_best(0);
        SearchResult { depth_reached: self.search_depth, mv, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits }
    }

    pub fn search_with_time(&mut self, board: &mut Board, wtime: u64, btime: u64, winc: u64, binc: u64) -> SearchResult {
        let (time, inc) = if board.us == WHITE {
            (wtime, winc)
        } else {
            (btime, binc)
        };

        let time = time / 50 + inc;
        self.search_to_time(board, time, true)
    }

    pub fn search_to_time(&mut self, board: &mut Board, time: u64, cut: bool) -> SearchResult {
        self.nodes_searched = 0;
        self.pv_table = PvTable::new(100);
        self.stop.store(false, Ordering::Relaxed);
        let time = time as u128;
        let start = Instant::now();
        let fallback_mv = board.mg.generate_moves(board).get_move(0).clone();

        thread::scope(|s| {
            let stop = Arc::clone(&self.stop);

            let handle = s.spawn(|| {
                let mut depth = 1;
                let mut best = Move::new_null_mv();
                while (start.elapsed().as_millis() <= time / 2 || !cut)
                && !self.stop.load(Ordering::Relaxed) {
                    let value = self.make_search(board, depth);
                    let mv = self.pv_table.get_best(0);
                    if !self.stop.load(Ordering::Relaxed) {
                        best = mv;
                    }
                    if value > MATE / 10 {
                        break;
                    }
                    depth += 1;
                }
                if best.is_null() {
                    let moves = board.mg.generate_moves(board);
                    best = moves.get_move(0).clone();
                }
                self.stop.store(true, Ordering::Relaxed);
                SearchResult { depth_reached: self.search_depth, mv: best, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits }
            });

            while start.elapsed().as_millis() < time && !stop.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(2));
            }
            stop.store(true, Ordering::Relaxed);
            let result = handle.join();
            match result {
                Ok(sr) => sr,
                Err(_) => SearchResult { mv: fallback_mv, nodes_searched: 0, depth_reached: 0, ttable_hits: 0 }
            }
        })
    }

    fn make_search(&mut self, board: &mut Board, depth: i32) -> i32 {
        self.generation += 1;
        self.search_depth = depth;
        self.ttable_hits = 0;
        let best_value = self.nega_max(board, 0, -INFINITY, INFINITY);
        if !self.stop.load(Ordering::Relaxed) {
            println!(
                "info depth {} cp {} tthits {} nodes searched {} pv {}",
                depth, best_value, self.ttable_hits, self.nodes_searched, self.pv_table.get_pv_string()
            );
        }
        best_value
    }

    pub fn nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
        if self.stop.load(Ordering::Relaxed) {
            return alpha;
        }
        if depth == self.search_depth {
            return self.quiesce_nega_max(board, depth, alpha, beta);
        }
        let depth_left = self.search_depth - depth;
        self.nodes_searched += 1;
        let hash = board.get_hash();

        if let Some(e) = self.ttable.probe(hash) {
            self.ttable_hits += 1;
            if e.depth_left >= depth_left {
                match e.entry_type {
                    TTEntryType::Exact => return e.score,
                    TTEntryType::Lower if e.score >= beta => return e.score,
                    TTEntryType::Upper if e.score <= alpha => return e.score,
                    _ => {}
                }
            }
        }


        let mut moves = board.mg.generate_moves(board);
        self.order_moves(&mut moves, depth);
        if moves.get_count() == 0 {
           return if board.is_check() { -MATE+depth } else { 0 }
        }

        let mut best_score = i32::MIN;
        for mv in moves.iter() {
            board.make_move(mv);
            let score = -self.nega_max(board, depth + 1, -beta, -alpha);
            board.unmake_move(mv);
            if score > best_score {
                best_score = score;
                self.pv_table.update(depth as usize, mv.clone());
                alpha = alpha.max(score);
            }
            if alpha >= beta { break } 
        }

       let tt_type = if best_score == alpha {TTEntryType::Lower}
                                      else if best_score == beta {TTEntryType::Upper}
                                      else {TTEntryType::Exact};
        self.store_tt(hash, best_score, depth_left, tt_type);
        best_score
    }

    fn store_tt(&mut self, hash: u64, score: i32, depth_left: i32, tt_type: TTEntryType) {
        let entry = Entry {
            key: hash,
            depth_left,
            entry_type: tt_type,
            generation: self.generation,
            score,
        };
        self.ttable.store(entry);
    }


    pub fn quiesce_nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop.load(Ordering::Relaxed) {
            return alpha;
        }
        self.nodes_searched += 1;
        let moves = board.mg.generate_moves(board);
        if moves.get_count() == 0 {
            if board.is_check() {
                return -MATE+depth;
            } else {
                return 0;
            }
        }
        let mut best_value = self.evaluator.evaluate(board);
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
            let score = -self.quiesce_nega_max(board, depth + 1, -beta, -alpha);
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

    fn order_moves(&self, move_list: &mut MoveList, ply: i32) {
        let best = self.pv_table.get_best(ply as usize);
        if best.is_null() {
            return;
        }
        let cnt = move_list.get_count();
        let moves = move_list.moves();
        for i in 0..cnt {
            if moves[i] == best {
                moves.swap(0, i);
                break;
            }
        }
    }

    pub fn get_nodes_searched(&self) -> i32 {
        self.nodes_searched
    }
}
