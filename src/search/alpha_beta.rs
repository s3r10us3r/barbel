use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{f64, thread};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use crate::evaluation::Evaluator;
use crate::moving::move_generation::{generate_moves, get_mg};
use crate::position::piece_set::PieceSet;
use crate::search::history::HistoryTable;
use crate::search::killers::KillerTable;
use crate::search::move_ordering::{OrderedMovesIter, QuiesceOrderedMovesIter};
use crate::search::transposition::TTEntryType;
use crate::{
    position::board::Board, 
    moving::mv::Move
};

use super::{
    transposition::{Entry, TTable},
};


const INFINITY: i32 = 10_000_000;
const MATE: i32 = 1_000_000;
const NULL_MOVE_RED: i32 = 3;


pub struct SearchResult {
    pub mv: Move,
    pub nodes_searched: u64,
    pub depth_reached: i32,
    pub ttable_hits: i32,
    pub nmp_hits: i32,
}


pub struct Searcher {
    ttable: TTable,
    history: HistoryTable,
    killers: KillerTable,
    evaluator: Evaluator,
    search_depth: i32,
    ttable_hits: i32,
    nodes_searched: u64,
    generation: i32,
    nmp_hits: i32,
    stop: Arc<AtomicBool>,
    lmr_table: [[i32; 64]; 218]
}

impl Default for Searcher {
    fn default() -> Self {
        Searcher {
            ttable: TTable::new(),
            evaluator: Evaluator::new(),
            search_depth: 0,
            ttable_hits: 0,
            nodes_searched: 0,
            generation: 0,
            nmp_hits: 0,
            stop: Arc::new(AtomicBool::new(false)),
            history: HistoryTable::new(),
            killers: KillerTable::new(),
            lmr_table: compute_lmr_table()
        }
    }
}

impl Searcher {
    pub fn new() -> Self {
        Self::default()
    }

    //test exclusive
    pub fn search_to_depth(&mut self, board: &mut Board, depth: i32) -> SearchResult {
        self.nodes_searched = 0;
        let mut best_mv = Move::null();
        for i in 1..(depth + 1) {
            (_, best_mv) = self.make_search(board, i);
        }
        SearchResult { depth_reached: self.search_depth, mv: best_mv, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits, nmp_hits: self.nmp_hits }
    }

    //test exclusive
    pub fn search_flat(&mut self, board: &mut Board, depth: i32) -> SearchResult {
        self.nodes_searched =0;
        let (_, mv) = self.make_search(board, depth);
        SearchResult { depth_reached: self.search_depth, mv, nodes_searched: self.nodes_searched, ttable_hits: self.ttable_hits, nmp_hits: self.nmp_hits }
    }


    pub fn prepare_search(&mut self, stop: Arc<AtomicBool>) {
        self.stop = stop;
        self.nodes_searched = 0;
        self.stop.store(false, Ordering::Relaxed);
    }

    //test exclusive
    pub fn search_to_time(&mut self, board: &mut Board, time: u64, cut: bool) -> SearchResult {
        self.nodes_searched = 0;
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
                    let (value, mv) = self.make_search(board, depth);
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


    pub fn make_search(&mut self, board: &mut Board, depth: i32) -> (i32, Move) {
        self.generation += 1;
        self.search_depth = depth;
        self.ttable_hits = 0;
        self.nmp_hits = 0;
        self.killers = KillerTable::new();
        let mut best_value = i32::MIN;
        let mut best_move = Move::null();

        let mut hash_move = Move::null();
        if let Some(e) = self.ttable.probe(board.get_hash()) {
            hash_move = e.best_move;
        }

        let mut ordered_moves = OrderedMovesIter::new(hash_move, depth);

        while let Some(mv) = ordered_moves.next(board, &self.history, &self.killers) {
            board.make_move(&mv);
            let score = -self.nega_max(board, 1, -INFINITY, INFINITY);
            if score > best_value {
                best_move = mv;
                best_value = score;
            }
            board.unmake_move(&mv);
        }
        

        if !self.stop.load(Ordering::Relaxed) {
            // let mut visited = vec![board.get_hash()];
            // let pv_string = self.get_pv_string(board, String::new(), &best_move, &mut visited);
            let pv_string = self.get_pv_string(board, &best_move);
            println!(
                "info depth {} cp {} tthits {} nodes searched {} nmp hits {} pv {}",
                depth, best_value, self.ttable_hits, self.nodes_searched, self.nmp_hits, pv_string
            )
        }
        (best_value, best_move)
    }

    fn get_pv_string(&mut self, board: &mut Board, best_mv: &Move) -> String {
        let mut visited = vec![];
        let mut mv_stack = vec![*best_mv];
        let mut s = best_mv.to_str() + " ";
        board.make_move(best_mv);

        loop {
            let hash = board.get_hash();
            let tt_entry = self.ttable.probe(hash);
            if tt_entry.is_none() || visited.contains(&hash) {
                break;
            }

            let mv = tt_entry.unwrap().best_move;
            s += (mv.to_str() + " ").as_str();
            board.make_move(&mv);
            mv_stack.push(mv);
            visited.push(hash);
        }

        while let Some(m) = mv_stack.pop() {
            board.unmake_move(&m);
        }

        s
    }

    fn nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.stop.load(Ordering::Relaxed) {
            return alpha;
        }
        if depth == self.search_depth {
            return self.quiesce_nega_max(board, depth, alpha, beta);
        }
        if depth > 0 && self.is_repetition(board) {
            return 0;
        }

        let org_alpha = alpha;
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
        let mut move_num = 0;

        while let Some(mv) = ordered_moves.next(board, &self.history, &self.killers) {
            board.make_move(&mv);
            let lmr = self.can_reduce(board, depth_left, depth, &mv, move_num);
            let score = if lmr > 0 {
                let temp_score = -self.nega_max(board, depth + 1 + lmr, -beta, -alpha);
                if temp_score > alpha {
                    -self.nega_max(board, depth + 1, -beta, -alpha)
                } else {
                    temp_score
                }
            } else {
                    -self.nega_max(board, depth + 1, -beta, -alpha)
            };
            
            board.unmake_move(&mv);

            move_num += 1;

            if score > best_score {
                best_score = score;
                alpha = alpha.max(score);
                best_move = mv;
            }
            if score >= beta {
                self.killers.update(depth, mv);
                self.history.add(&mv, depth_left);
            }
            if alpha >= beta { break } 
        }

        //no moves
        if best_move.is_null() {
           return if board.is_check() { -MATE+depth } else { 0 }
        }

       let tt_type = if best_score > org_alpha && best_score < beta {TTEntryType::Exact}
                                      else if best_score >= beta {TTEntryType::Lower}
                                      else {TTEntryType::Upper};
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

    fn is_repetition(&self, board: &Board) -> bool {
        let hash = board.get_hash();
        let hash_stack = board.get_hash_stack();
        hash_stack.contains(&hash)
    }

    fn is_in_zugzwang(&self, pieces: &PieceSet) -> bool {
        pieces.get_orthogonals() | pieces.get_diagonals() | pieces.get_knights() == 0
    }

    fn quiesce_nega_max(&mut self, board: &mut Board, depth: i32, mut alpha: i32, beta: i32) -> i32 {
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
        let mut best_value = self.evaluator.evaluate(board, get_mg());
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

    pub fn get_nodes_searched(&self) -> u64 {
        self.nodes_searched
    }

    fn can_reduce(&self, board: &Board, depth_left: i32, depth: i32, mv: &Move, move_num: i32) -> i32 {
        if depth_left > 3 && !mv.is_non_quiet() &&  move_num > 3 && !self.killers.is_killer(depth, mv) && board.get_checkers() == 0 {
            let depth_capped = std::cmp::min(depth_left, 63) as usize;
            let red_depth = self.lmr_table[move_num as usize][depth_capped];
            std::cmp::min(red_depth, depth_left - 1)
        } else {
            0
        }
    }
}

fn compute_lmr_table() -> [[i32; 64]; 218] {
    let mut depth = 0;
    let mut lmr_arr = [[0; 64]; 218];
    loop {
        if depth >= 64 {
            break;
        }
        let mut mv_num = 0;
        loop {
            if mv_num >= 218 {
                break;
            }
            let lmr_depth = 0.99 + (depth as f64).ln() * (mv_num as f64).ln() / f64::consts::PI;
            lmr_arr[mv_num][depth] = lmr_depth as i32;
            mv_num += 1;
        }
        depth += 1;
    }
    lmr_arr
}
