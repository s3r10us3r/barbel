use uci::uci_interpreter::UciController;

pub mod attacks;
pub mod bitboard_helpers;
pub mod position;
pub mod constants;
pub mod evaluation;
pub mod fen_parsing;
pub mod lookups;
pub mod moving;
pub mod search;
pub mod uci;
pub mod tests;
pub mod attack_mapper;


pub fn run() {
    let mut uci_controller = UciController::new();
    uci_controller.run();
}
