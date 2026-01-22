use crate::{fen_parsing::parse_fen::parse_fen, position::board::Board};

pub struct Epd {
    pub position: Board,
}

impl Epd {
    pub fn new(epd: &str) -> Self {
        let space_splits: Vec<&str> = epd.split(" ").collect();
        let fen = space_splits[..4].join(" ");
        let board = parse_fen(&fen).unwrap();
        Epd { position: board }
    }
}

