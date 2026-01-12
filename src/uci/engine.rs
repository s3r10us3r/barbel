use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{scope, sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

use crate::constants::WHITE;
use crate::fen_parsing::parse_fen::{parse_fen, FenError};
use crate::moving::move_generation::get_mg;
use crate::moving::move_list::MoveList;
use crate::position::board::Board;
use crate::position::zobrist_hashing::ZobristHasher;
use crate::search::alpha_beta::Searcher;

//this holds global state
pub struct Engine {
    board: Board,
    mvs: MoveList,
    searcher: Option<Searcher>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<Searcher>>
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            board: Board::new(ZobristHasher::new()),
            mvs: MoveList::new(),
            searcher: Some(Searcher::new()),
            stop: Arc::new(AtomicBool::new(false)),
            thread: None
        }
    }
}

#[derive(Clone, Copy)]
enum SearchLimit {
    Depth(i32),
    Nodes(u64),
    Time(u128),
    Infinite,
}

impl SearchLimit {
    fn soft_stop(&self, nodes_searched: u64, depth_reached: i32, time_passed: u128) -> bool {
        match self {
            Self::Infinite => false,
            Self::Depth(num) => depth_reached > *num,
            Self::Nodes(nodes) => nodes_searched >= *nodes,
            Self::Time(t) => time_passed >= *t / 2,
        }
    }

    fn hard_stop(&self, time_passed: u128) -> bool {
        match self {
            Self::Infinite => false,
            Self::Depth(_) => false,
            Self::Nodes(_) => false,
            Self::Time(t) => time_passed >= *t,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stop(&mut self) {
        let t = self.thread.take();
        if let Some(th) = t {
            self.stop.store(true, Ordering::Relaxed);
            let res = th.join();
            match res {
                Ok(s) => self.searcher = Some(s),
                _ => panic!("Error in search")
            }
        }
    }

    pub fn set_pos(&mut self, fen: &str) -> Result<(), FenError> {
        self.mvs.reset();
        let new_board = parse_fen(fen)?;
        self.board = new_board;
        Ok(())
    }

    pub fn make_move(&mut self, mv_s: &str) -> Result<(), String> {
        self.mvs = get_mg().generate_moves(&self.board);
        for mv in self.mvs.iter() {
            let mv_str = mv.to_str();
            if mv_str == mv_s {
                self.board.make_move(mv);
                return Ok(());
            }
        }
        Err(format!("Move {mv_s} not found"))
    }

    pub fn search_movetime(&mut self, movetime: u64) {
        let time = movetime as u128;
        let sl = SearchLimit::Time(time);
        self.run(sl);
    }

    pub fn search_nodes(&mut self, nodes: u64) {
        let sl = SearchLimit::Nodes(nodes);
        self.run(sl);
    }

    pub fn search_to_depth(&mut self, depth: i32) {
        let sl = SearchLimit::Depth(depth);
        self.run(sl);
    }

    pub fn search_with_time(&mut self, wtime: u64, btime: u64, winc: u64, binc: u64) {
        let sl = if self.board.us == WHITE {
            self.calc_time(wtime, winc)
        } else {
            self.calc_time(btime, binc)
        };
        self.run(sl);
    }

    pub fn search_infinite(&mut self) {
        let sl = SearchLimit::Infinite;
        self.run(sl);
    }

    fn calc_time(&self, time: u64, inc: u64) -> SearchLimit {
        let to_use = (time / 20 + inc / 2) as u128;
        SearchLimit::Time(to_use)
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(t) = &self.thread {
            !&t.is_finished()
        } else {
            false
        }
    }

    fn run(&mut self, limit: SearchLimit) {
        if self.is_running() {
            return;
        }
        self.stop();

        if self.searcher.is_some() {
            let mut searcher = self.searcher.take().unwrap();
            self.stop = Arc::new(AtomicBool::new(false));
            let stop = self.stop.clone();
            let mut best_mv = *get_mg().generate_moves(&self.board).get_move(0);
            let mut board = self.board.clone();
            let t = spawn(move || {
                searcher.prepare_search(stop.clone());
                let searcher = scope(|s| {
                    let start = Instant::now();
                    let stop = stop.clone();
                    let stop2 = stop.clone();
                    let handle = s.spawn(move || {
                        let mut nodes_searched = 0;
                        let mut depth= 1;
                        while !limit.soft_stop(nodes_searched, depth, start.elapsed().as_millis()) 
                        && !stop2.load(std::sync::atomic::Ordering::Relaxed) 
                        {
                            searcher.make_search(&mut board, depth);
                            if !stop2.load(Ordering::Relaxed) {
                                best_mv = searcher.get_best();
                                depth += 1;
                                nodes_searched += searcher.get_nodes_searched();
                            }
                        }
                        (searcher, best_mv)
                    });
                    while !limit.hard_stop(start.elapsed().as_millis()) && !stop.load(Ordering::Relaxed) && !handle.is_finished() {
                        sleep(Duration::from_millis(2));
                    }
                    stop.store(true, Ordering::Relaxed);
                    let (searcher, best_mv) = handle.join().unwrap();
                    println!("bestmove {}", best_mv.to_str());
                    searcher
                });
                searcher
            });
            self.thread = Some(t);
        }
    }


    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}
