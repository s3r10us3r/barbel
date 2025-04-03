use uci::uci_interpreter::UciController;

mod attacks;
mod bitboard_helpers;
mod board;
mod constants;
mod fen_parsing;
mod lookups;
mod moving;
mod uci;

pub fn run() {
    println!("Barbel 0.1 by s3r10us3r");
    let mut uci_controller = UciController::new();
    uci_controller.run();
}
