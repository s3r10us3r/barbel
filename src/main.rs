use std::env;

use barbel::run_perft;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Expected 2 arguments <fen> <depth>");
        return;
    }
    let fen = &args[1];
    let depth: i32 = args[2].parse().unwrap();
    run_perft(fen, depth);
}
