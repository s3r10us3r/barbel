use perft::make_perft;

mod attack_maps;
mod bitboard_helpers;
mod board;
mod board_state;
mod constants;
mod fen_parsing;
mod legality;
mod lookups;
mod move_generation;
mod move_making;
mod mv;
mod parse_to_fen;
mod perft;
mod piece_set;

pub fn run_perft(fen: &str, depth: i32) {
    let result = make_perft(fen, depth);
    match result {
        Ok(r) => println!("Result: {}, Time: {}", r.result, r.time),
        Err(e) => println!("ERROR WHEN PARSING FEN: {:?}", e),
    };
}
