use crate::{
    board::{board::Board, zobrist_hashing::ZobristHasher},
    fen_parsing::fen_parsing::parse_fen,
    moving::move_generation::{generate_moves, MoveList},
    search::alpha_beta::Searcher,
};

pub struct StateError {
    message: String,
}

impl StateError {
    pub fn new(msg: &str) -> Self {
        StateError {
            message: msg.to_string(),
        }
    }
}

//this holds global state
pub struct Engine {
    board: Board,
    mvs: MoveList,
    searcher: Searcher,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::new(ZobristHasher::new()),
            mvs: MoveList::new(),
            searcher: Searcher::new(),
        }
    }

    pub fn set_pos(&mut self, fen: &str) -> Result<(), StateError> {
        self.mvs.reset();
        let fen_res = parse_fen(fen);
        let new_board = match fen_res {
            Ok(b) => b,
            Err(_) => return Err(StateError::new("Invalid fen")),
        };
        self.board = new_board;
        Ok(())
    }

    pub fn make_move(&mut self, mv_s: &str) -> Result<(), StateError> {
        self.mvs.reset();
        generate_moves(&mut self.mvs, &self.board);
        for mv in self.mvs.iter() {
            let mv_str = mv.to_str();
            if mv_str == mv_s {
                self.board.make_move(mv);
                return Ok(());
            }
        }
        Err(StateError::new(&format!("Move {} not found", mv_s)))
    }

    pub fn search_to_depth(&mut self, depth: i32) {
        let mv = self.searcher.search_to_depth(&mut self.board, depth);
        println!("bestmove {}", mv.to_str());
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}
