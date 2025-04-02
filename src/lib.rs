use perft::make_perft;

mod attacks;
mod bitboard_helpers;
mod board;
mod constants;
mod fen_parsing;
mod lookups;
mod moving;
mod perft;

pub fn run_perft(fen: &str, depth: i32) {
    let result = make_perft(fen, depth);
    match result {
        Ok(r) => println!("Result: {}, Time: {}", r.result, r.time),
        Err(e) => println!("ERROR WHEN PARSING FEN: {:?}", e),
    };
}
