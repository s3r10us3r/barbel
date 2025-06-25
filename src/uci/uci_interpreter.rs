use super::engine::Engine;
use crate::{tests::wac::wac_test, uci::perft::make_perft};
use std::{
    io::{self, Write},
    process::exit,
};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct UciController {
    engine: Engine,
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
        UciController { engine }
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
                    self.exec_command(&input);
                }
            }
        }
    }

    fn exec_command(&mut self, command: &str) {
        let commands: Vec<&str> = command.trim().split(" ").collect();
        let command = commands[0];
        let args: Vec<&str> = commands[1..].to_vec();
        match command {
            "go" => self.go(args),
            "position" => self.position(args),
            "isready" => self.is_ready(),
            "stop" => self.stop(),
            "help" => self.help(),
            "uci" => self.uci(),
            "quit" => self.quit(),
            "ucinewgame" => self.ucinewgame(),
            "wac" => wac_test(),
            _ => self.invalid_command(command),
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

    fn go(&mut self, args: Vec<&str>) {
        if args[0] == "perft" {
            self.go_perft(args);
        } else if args[0] == "depth" {
            self.go_depth(args);
        } else if args[0] == "wtime" {
            self.go_time(args);
        } else if args[0] == "movetime" {
        }
    }

    fn go_perft(&mut self, args: Vec<&str>) {
        let parse_result = args[1].parse::<i32>();
        if let Ok(depth) = parse_result {
            let board = self.engine.get_board_mut();
            let result = make_perft(depth, board);
            print!("\nNodes searched: {}\n\n", result.result);
        } else {
            println!("Invalid argument!\n");
        }
    }

    fn go_depth(&mut self, args: Vec<&str>) {
        let parse_result = args[1].parse::<i32>();
        if let Ok(depth) = parse_result {
            if depth <= 0 {
                println!("Invalid depth {}, depth must be at least 1", { depth });
            }
            self.engine.search_to_depth(depth);
        } else {
            println!("Invalid depth: {}", args[1]);
        }
    }

    fn go_time(&mut self, args: Vec<&str>) {
        let mut wtime: u64 = 0;
        let mut btime: u64 = 0;
        let mut winc: u64 = 0;
        let mut binc: u64 = 0;

        for i in (0..args.len()-1).step_by(2) {
            let s = args[i];
            let tr = args[i+1].parse::<u64>();
            if let Ok(t) = tr {
                match s {
                    "wtime" => wtime = t,
                    "btime" => btime = t,
                    "winc" => winc = t,
                    "binc" => binc = t,
                    _ => panic!("Invallid arguments")
                }
            } else {
                panic!("Invalid arguments")
            }
        }
        self.engine.search_with_time(wtime, btime, winc, binc);
    }
            
    fn go_movetime(&mut self, args: Vec<&str>) {
        let movetime = args[1].parse::<u64>();
    }

    fn position(&mut self, args: Vec<&str>) {
        match args[0] {
            "startpos" => {
                let res = self.engine.set_pos(START_POS);
                if let Err(e) = res {
                    println!("Invalid fen!");
                    println!("{e:?}");
                }
                if args.len() > 2 && args[1] == "moves" {
                    for mv_s in args[2..].iter() {
                        let e = self.engine.make_move(mv_s);
                        if e.is_err() {
                            println!("MOVE NOT FOUND: {mv_s}");
                        }
                    }
                }
            }
            "fen" => {
                if args.len() <= 4 {
                    println!("Invalid fen!");
                    println!("too little arguments")
                }
                let fen_str = args[1..].join(" ");
                let splits: Vec<&str> = fen_str.split("moves").collect();
                let (fen_str, moves_opt) = if splits.len() > 2 {
                    (splits[0].trim(), Some(splits[1]))
                } else {
                    (splits[0], None)
                };
                let res = self.engine.set_pos(fen_str);
                if let Err(e) = res {
                    println!("Invalid fen!");
                    println!("{e:?}");
                    return;
                }
                if let Some(moves_str) = moves_opt {
                    for mv_s in moves_str.trim().split(' ') {
                        let _ = self.engine.make_move(mv_s);
                    }
                }
            }
            _ => println!("Unrecognized argument"),
        }
    }

    fn is_ready(&self) {
        println!("readyok");
    }

    fn stop(&self) {
        unimplemented!();
    }

    fn help(&self) {
        println!("BARBEL, THE BEST CHESS ENGINE");
    }

    fn invalid_command(&self, command: &str) {
        println!("Unknown command: '{command}'. Type help for more information.");
    }
}
