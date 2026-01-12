use regex::Regex;

use super::engine::Engine;
use crate::{tests::{itflat::make_comp_tests, nps::make_nps, test_suites::NOLOT, transpositions::test_transpositions, wac::wac_test}, uci::perft::make_perft};
use std::{
    io::{self, Write}, process::exit
};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


pub struct UciController {
    engine: Engine,
    parsed_command: Option<String>
}

impl Default for UciController {
    fn default() -> Self {
        UciController::new()
    }
}

impl UciController {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        let _ = engine.set_pos(START_POS);
        UciController { engine, parsed_command: None }
    }

    pub fn run(&mut self) {
        loop {
            let mut input = String::new();
            io::stdout().flush().unwrap();
            let rl = io::stdin().read_line(&mut input);
            match rl {
                Err(e) => {
                    println!("INPUT ERROR: {e:?}");
                    return;
                }
                Ok(us) => {
                    if us == 0 {
                        return;
                    }
                    let normalized_command = merge_spaces(input);
                    self.exec_command(normalized_command);
                }
            }
        }
    }

    fn exec_command(&mut self, command: String ) {
        self.parsed_command = Some(command);
        let token = self.pop_token();
        if let Some(t) = token {
            match t.as_str() {
                "go" => self.go(),
                "position" => self.position(),
                "isready" => self.is_ready(),
                "stop" => self.stop(),
                "help" => self.help(),
                "uci" => self.uci(),
                "quit" => self.quit(),
                "ucinewgame" => self.ucinewgame(),
                "wac" => wac_test(),
                "nps" => nps_test(),
                "tt_test"=> self.tt_test(),
                "iter_test" => self.iter_test(),
                _ => self.invalid_command(&t),
            }
        }
    }

    fn tt_test(&mut self) {
        let token = self.pop_token();
        match token {
            Some(t) => {
                let num = t.parse::<i32>();
                if let Ok(n) = num {
                    let board = self.engine.get_board_mut();
                    let (false_positives, false_negatives) = test_transpositions(board, n);
                    println!("Found {false_positives} false positives and {false_negatives} false negatives at depth {n}");
                } else {
                    println!("Invalid argument");
                }
            }
            _ => println!("Please provide an argument!")
        }
    }

    fn iter_test(&mut self) {
        make_comp_tests();
    }

    fn go(&mut self) {
        let token = self.pop_token();
        match token {
            Some(t) => {
                match t.as_str() {
                    "perft" => self.go_perft(),
                    "depth" => self.go_depth(),
                    "wtime" | "btime" | "winc" | "binc" => {
                        self.push_token_to_front(t);
                        self.go_time();
                    }
                    "movetime" => self.go_movetime(),
                    _ => panic!()
                }
            }
            None => {
                self.go_infinite();
            }
        }
    }

    fn go_infinite(&mut self) {
        self.engine.search_infinite();
    }

    fn go_perft(&mut self) {
        let token = self.pop_token();
        match token {
            Some(t) => {
                let parse_result = t.parse::<i32>();
                if let Ok(depth) = parse_result {
                    let board = self.engine.get_board_mut();
                    let result = make_perft(depth, board);
                    println!("\nNodes searched: {}, Time: {}\n\n", result.result, result.time);
                } else {
                    println!("Invalid argument!\n");
                }
            }
            None => {
                self.reset();
            }
        }
    }

    fn go_depth(&mut self) {
        let token = self.pop_token();
        match token {
            Some(t) => {
                let parse_result = t.parse::<i32>();
                if let Ok(depth) = parse_result {
                    if depth <= 0 {
                        println!("Invalid depth {}, depth must be at least 1", { depth });
                        self.reset();
                    }
                    self.engine.search_to_depth(depth);
                } else {
                    println!("Invalid argument!\n");
                }
            }
            None => {
                self.reset();
            }
        }
    }

    fn go_movetime(&mut self) {
        if let Some(num) = self.pop_token() {
            let time_res = num.parse();
            if let Ok(time) = time_res {
                self.engine.search_movetime(time);
            }
        }
    }


    fn go_time(&mut self) {
        let mut wtime: u64 = 0;
        let mut btime: u64 = 0;
        let mut winc: u64 = 0;
        let mut binc: u64 = 0;

        for _ in 0..4 {
            if let (Some(t), Some(val)) = (self.pop_token(), self.pop_token()) {
                let num = val.parse::<u64>().unwrap_or_default();
                match t.as_str() {
                    "wtime" => wtime = num,
                    "btime" => btime = num,
                    "winc" => winc = num,
                    "binc" => binc = num,
                    _ => {}
                }
            } else {
                panic!("Invalid args")
            }
        }
        self.engine.search_with_time(wtime, btime, winc, binc);
    }

    fn position(&mut self) {
        let token = self.pop_token();
        if let Some(p) = token {
            match p.as_str() {
                "startpos" => self.position_startpos(),
                "fen" => self.position_fen(),
                _ => {
                    self.reset();
                }
            }
        } else {
            self.reset();
        }
    }

    fn position_startpos(&mut self) {
        let token = self.pop_token();
        match token {
            Some(t) => {
                let mut moves = vec![];
                if t.as_str() == "moves" {
                    loop {
                        let token = self.pop_token();
                        if let Some(t) = token {
                            moves.push(t);
                        } else {
                            break;
                        }
                    }
                    self.position_with_fen(START_POS.to_owned(), moves);
                }
            } 
            None => self.position_with_fen(START_POS.to_owned(), vec![]),
        }
    }

    fn position_fen(&mut self) {
        let mut fen_vec = Vec::new();
        loop {
            let token = self.pop_token();
            match token {
                Some(t) => {
                    match t.as_str() {
                        "moves" => {
                            let moves = self.parse_moves();
                            self.position_with_fen(fen_vec.join(" "), moves);
                            break;
                        },
                        _ => fen_vec.push(t.to_owned()),
                    }
                }
                None => {
                    self.position_with_fen(fen_vec.join(" "), vec![]);
                    break;
                }
            }
        }

    }

    //this assumes the 'moves' string is already parsed
    fn parse_moves(&mut self) -> Vec<String> {
        let mut moves = vec![];
        loop {
            let token = self.pop_token();
            match token {
                Some(t) => {
                    moves.push(t);
                }
                None => { break; }
            }
        }
        moves
    }

    fn position_with_fen(&mut self, fen: String, moves: Vec<String>) {
        let res = self.engine.set_pos(fen.as_str());
        if let Err(e) = res {
            println!("Invalid fen!");
            println!("{e:?}");
            self.reset();
            return;
        }
        for move_str in moves {
            let mv_s = move_str.as_str();
            let err = self.engine.make_move(mv_s);
            if let Err(e) = err {
                panic!("Invalid move! {e:?}");
            }
        }
    }

    fn reset(&mut self) {
        self.parsed_command = None;
    }

    fn pop_token(&mut self) -> Option<String> {
        let parsed_command = self.parsed_command.clone();
        match parsed_command {
            Some(command) => {
                match command.split_once(' ') {
                    Some((next, rest)) => {
                        if next.is_empty() {
                            self.parsed_command = None;
                            None
                        } else {
                            self.parsed_command = Some(rest.to_owned());
                            Some(next.to_owned())
                        }
                    }
                    None => {
                        if command.is_empty() { None } 
                        else { 
                            self.parsed_command = None;
                            Some(command.to_owned()) 
                        }
                    }
                }
            },
            None => None
        }
    }

    fn push_token_to_front(&mut self, token: String) {
        let parsed_command = self.parsed_command.clone();
        match parsed_command {
            Some(s) => self.parsed_command = Some(token + " " + s.as_str()),
            None => self.parsed_command = Some(token),
        }
    }

    fn ucinewgame(&mut self) {
        self.engine = Engine::new();
        _ = self.engine.set_pos(START_POS);
    }

    fn quit(&mut self) {
        exit(0);
    }

    fn uci(&mut self) {
        println!("uciok");
    }

    fn is_ready(&mut self) {
        if !self.engine.is_running() {
            println!("readyok");
        }
    }

    fn stop(&mut self) {
        self.engine.stop();
    }

    fn help(&self) {
        println!("BARBEL, THE BEST CHESS ENGINE");
    }

    fn invalid_command(&self, command: &str) {
        println!("Unknown command: '{command}'. Type help for more information.");
    }
}

fn merge_spaces(s: String) -> String {
    let s = s.trim();
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(s, " ").to_string()
}

fn nps_test() {
    let nps_result = make_nps(NOLOT);
    let nps = nps_result.nodes as f64 / (nps_result.time as f64 / 1000.);
    println!("\nNodes searched: {}\nTime measured: {:.2}s\nNodes per second: {:.2}", nps_result.nodes, nps_result.time as f64 / 1000., nps);
}
