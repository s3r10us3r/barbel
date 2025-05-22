use uci::uci_interpreter::UciController;

pub mod attacks;
pub mod bitboard_helpers;
pub mod board;
pub mod constants;
pub mod evaluation;
pub mod fen_parsing;
pub mod lookups;
pub mod moving;
pub mod search;
pub mod uci;
pub mod tests;

pub fn run() {
    println!("Barbel 0.22 by s3r10us3r");
    let mut uci_controller = UciController::new();
    uci_controller.run();
}
