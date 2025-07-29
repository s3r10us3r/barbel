use crate::{fen_parsing::parse_fen::parse_fen, position::board::Board};
use std::collections::HashMap;

// represents the extended position notation 
// panics on wrong input, should not be used beyond tests
// https://www.chessprogramming.org/Extended_Position_Description
pub struct Epd {
    pub position: Board,
    pub opcodes: HashMap<String, String>,
}

impl Epd {
    pub fn new(epd: &str) -> Self {
        let space_splits: Vec<&str> = epd.split(" ").collect();
        let fen = space_splits[..4].join(" ");
        let opcode_splits: Vec<&str> = space_splits[4..].to_owned();
        let opcode_string: String =  opcode_splits.join(" ");
        let opcode_pairs: Vec<&str> = opcode_string.split(";")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut opcodes: HashMap<String, String> = HashMap::new();
        for pair in opcode_pairs {
            let pair: Vec<&str> = pair.split(" ").collect();
            let opcode = pair[0].to_owned();
            let value = pair[1].to_owned();
            opcodes.insert(opcode, value);
        }

        let board = parse_fen(&fen).unwrap();
        Epd { position: board, opcodes }
    }

    pub fn get_opcode(&self, opcode: &str) -> Option<&String> {
        self.opcodes.get(opcode)
    }
}

#[cfg(test)]
mod test {
    use crate::tests::epd::Epd;

    #[test]
    fn name() {
        let epd = String::from("1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - - bm Qd1+; id \"BK.01\";");
        let epd = Epd::new(&epd);
        assert_eq!(epd.get_opcode("bm").unwrap(), "Qd1+");
    }
}
