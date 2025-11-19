use std::f32::consts::PI;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use crate::evaluation::Evaluator;
use crate::moving::move_generation::generate_moves;
use crate::position::piece_set::PieceSet;
use crate::search::history::HistoryTable;
use crate::search::killers::KillerTable;
use crate::search::move_ordering::{OrderedMovesIter, QuiesceOrderedMovesIter};
use crate::search::transposition::TTEntryType;
use crate::{
    position::board::Board, constants::WHITE, 
    moving::mv::Move
};

use super::{
    pv_table::PvTable,
    transposition::{Entry, TTable},
};


const INFINITY: i32 = 10_000_000;
const MATE: i32 = 1_000_000;
const NULL_MOVE_RED: i32 = 3;


pub struct SearchResult {
    pub mv: Move,
    pub nodes_searched: i32,
    pub depth_reached: i32,
    pub ttable_hits: i32,
    pub nmp_hits: i32,
}


pub struct Searcher {
    ttable: TTable,
    pv_table: PvTable,
    history: HistoryTable,
    killers: KillerTable,
    evaluator: Evaluator,
    search_depth: i32,
    ttable_hits: i32,
    nodes_searched: i32,
    generation: i32,
    nmp_hits: i32,
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
            nmp_hits: 0,
            stop: Arc::new(AtomicBool::new(false)),
            history: HistoryTable::new(),
            killers: KillerTable::new(),
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
        SearchResult { depth_reached: self.search_depth, mv, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits, nmp_hits: self.nmp_hits }
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
        let fallback_mv = *generate_moves(board).get_move(0);

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
                    let moves = generate_moves(board);
                    best = *moves.get_move(0);
                }
                self.stop.store(true, Ordering::Relaxed);
                SearchResult { depth_reached: self.search_depth, mv: best, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits, nmp_hits: self.nmp_hits }
            });

            while start.elapsed().as_millis() < time && !stop.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(2));
            }
            stop.store(true, Ordering::Relaxed);
            let result = handle.join();
            match result {
                Ok(sr) => sr,
                Err(_) => SearchResult { mv: fallback_mv, nodes_searched: 0, depth_reached: 0, ttable_hits: 0 , nmp_hits: 0 }
            }
        })
    }

    fn make_search(&mut self, board: &mut Board, depth: i32) -> i32 {
        self.generation += 1;
        self.search_depth = depth;
        self.ttable_hits = 0;
        self.nmp_hits = 0;
        self.killers = KillerTable::new();
        let best_value = self.nega_max(board, 0, -INFINITY, INFINITY);
        if !self.stop.load(Ordering::Relaxed) {
            println!(
                "info depth {} cp {} tthits {} nodes searched {} nmp hits {} pv {}",
                depth, best_value, self.ttable_hits, self.nodes_searched, self.nmp_hits, self.pv_table.get_pv_string()
            );
        }
        best_value
    }

    pub fn nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop.load(Ordering::Relaxed) {
            return alpha;
        }
        if depth == self.search_depth {
            return self.quiesce_nega_max(board, depth, alpha, beta);
        }
        let depth_left = self.search_depth - depth;
        self.nodes_searched += 1;
        let hash = board.get_hash();
        
        let mut hash_move = Move::null();
        if let Some(e) = self.ttable.probe(hash) {
            hash_move = e.best_move;
            self.ttable_hits += 1;
            if depth > 0 && e.depth_left >= depth_left {
                match e.entry_type {
                    TTEntryType::Exact => {
                        return e.score
                    },
                    TTEntryType::Lower if e.score >= beta => { 
                        return e.score
                    },
                    TTEntryType::Upper if e.score <= alpha => {
                        return e.score
                    }
                    _ => {}
                }
            }
        }

        
        // null move reduction
        if !self.is_in_zugzwang(board.get_ally_pieces()) && depth_left > NULL_MOVE_RED && !board.is_check() {
            board.make_null_mv();
            let nmr_score = -self.nega_max(board, depth + NULL_MOVE_RED, -beta, -beta + 1);
            board.unmake_null_move();
            if nmr_score >= beta {
                self.nmp_hits += 1;
                return beta;
            }
        }

        let mut ordered_moves = OrderedMovesIter::new(hash_move, depth);
        let mut best_score = i32::MIN;
        let mut best_move = Move::null();

        let mut mv_num = 0;
        while let Some(mv) = ordered_moves.next(board, &self.history, &self.killers) {
            let lmr = self.compute_lmr(board, &mv, mv_num, depth, depth_left);
            if lmr > 0 {
                board.make_move(&mv);
                let lmr_score = -self.nega_max(board, depth + 1 + lmr, -beta, -alpha);
                board.unmake_move(&mv);
                if lmr_score < alpha { continue; }
            }

            board.make_move(&mv);
            let score = -self.nega_max(board, depth + 1, -beta, -alpha);
            board.unmake_move(&mv);

            self.history.add(&mv, depth_left);
            if score > best_score {
                best_score = score;
                self.pv_table.update(depth as usize, mv);
                alpha = alpha.max(score);
                best_move = mv;
            }
            if score >= beta {
                self.killers.update(depth, mv);
            }
            if alpha >= beta { break } 
            mv_num += 1;
        }

        //no moves
        if best_move.is_null() {
           return if board.is_check() { -MATE+depth } else { 0 }
        }

       let tt_type = if best_score == alpha {TTEntryType::Lower}
                                      else if best_score == beta {TTEntryType::Upper}
                                      else {TTEntryType::Exact};
        self.store_tt(hash, best_score, depth_left, tt_type, best_move);
        best_score
    }

    fn store_tt(&mut self, hash: u64, score: i32, depth_left: i32, tt_type: TTEntryType, best_move: Move) {
        let entry = Entry {
            key: hash,
            depth_left,
            entry_type: tt_type,
            generation: self.generation,
            score,
            best_move
        };
        self.ttable.store(entry);
    }

    fn is_in_zugzwang(&self, pieces: &PieceSet) -> bool {
        pieces.get_orthogonals() | pieces.get_diagonals() | pieces.get_knights() == 0
    }

    pub fn quiesce_nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop.load(Ordering::Relaxed) {
            return alpha;
        }
        self.nodes_searched += 1;
        let moves = generate_moves(board);
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

        let mut ordered_moves = QuiesceOrderedMovesIter::new(moves, board, &self.history, &self.killers, depth);
        while let Some(mv) = ordered_moves.next() {
            board.make_move(&mv);
            let score = -self.quiesce_nega_max(board, depth + 1, -beta, -alpha);
            board.unmake_move(&mv);
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

    pub fn get_nodes_searched(&self) -> i32 {
        self.nodes_searched
    }

    fn compute_lmr(&self, board: &Board, mv: &Move, move_num: i32, depth: i32, depth_left: i32) -> i32 {
        if board.is_check() || self.killers.is_killer(depth, mv) || mv.is_capture() || mv.is_promotion() || depth_left <= 3 || move_num < 10 {
            0
        } else {
            let depth_left_f = depth_left as f32;
            let move_num_f = move_num as f32;
            let red = 0.99 + depth_left_f.ln() * move_num_f.ln() / PI;
            let red_i = red as i32;
            red_i.min(depth_left)
        }
    }
}
