use super::engine::Engine;
use crate::uci::perft::make_perft;
use std::io::{self, Write};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct UciController {
    engine: Engine,
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
                    println!("INPUT ERROR: {:?}", e);
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
            _ => self.invalid_command(command),
        }
    }

    fn go(&mut self, args: Vec<&str>) {
        if args[0] == "perft" {
            self.go_perft(args);
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

    fn position(&mut self, args: Vec<&str>) {
        match args[0] {
            "startpos" => {
                let res = self.engine.set_pos(START_POS);
                if let Err(_) = res {
                    println!("Invalid fen!");
                }
                if args.len() > 2 && args[1] == "moves" {
                    for mv_s in args[2..].iter() {
                        let _ = self.engine.make_move(mv_s);
                    }
                }
            }
            "fen" => {
                if args.len() <= 5 {
                    println!("Invalid fen!");
                }
                let fen_str = args[1..7].join(" ");
                let res = self.engine.set_pos(&fen_str);
                if let Err(_) = res {
                    println!("Invalid fen!");
                    return;
                }
                if args.len() > 8 && args[8] == "moves" {
                    for mv_s in args[6..].iter() {
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
        println!(
            "Unknown command: '{}'. Type help for more information.",
            command
        );
    }
}
