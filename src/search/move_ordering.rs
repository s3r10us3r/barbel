use crate::{evaluation::piece_values::MIDGAME_PIECE_VALUES, moving::{move_list::MoveList, mv::Move}, position::board::Board};

pub struct OrderedMovesIter {
    moves: MoveList,
    phase: MoveOrderingPhase,
    curr_idx: usize,
    pv_node: Option<Move>,
}

enum MoveOrderingPhase {
    PvNode,
    WinningCapturesPromotions,
    EqualCaptures,
    LosingCaptures,
    Quiet,
}

impl OrderedMovesIter {
    pub fn new(mut moves: MoveList, pv_node: Move) -> Self {
        let mut pv_node_u = None;
        if !pv_node.is_null() {
            for i in 0..moves.get_count() {
                if *moves.get_move(i) == pv_node {
                    moves.remove(i);
                    pv_node_u = Some(pv_node);
                    break;
                }
            }
        }
        OrderedMovesIter { moves, phase: MoveOrderingPhase::PvNode, curr_idx: 0, pv_node: pv_node_u }
    }

    fn next_pv_node(&mut self, board: &Board) -> Option<Move> {
        self.phase = MoveOrderingPhase::WinningCapturesPromotions;
        if self.pv_node.is_some() {
            self.pv_node.clone()
        } else {
            self.next(board)
        }
    }

    fn next_winning_cap(&mut self, board: &Board) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if mv.is_promotion() || (mv.is_capture() && cap_score(mv, board) > 0) {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            } 
            self.curr_idx += 1;
        }
        self.curr_idx = 0;
        self.phase = MoveOrderingPhase::EqualCaptures;
        self.next(board)
    } 

    fn next_equal_cap(&mut self, board: &Board) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if mv.is_capture() && cap_score(mv, board) == 0 {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            } 
            self.curr_idx += 1;
        }
        self.curr_idx = 0;
        self.phase = MoveOrderingPhase::Quiet;
        self.next(board)
    }

    fn next_quiet(&mut self, board: &Board) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if !mv.is_non_quiet() {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            }
            self.curr_idx += 1;
        }
        self.curr_idx = 0;
        self.phase = MoveOrderingPhase::LosingCaptures;
        self.next(board)
    }

    fn next_non_filter(&mut self) -> Option<Move> {
        if self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            self.curr_idx += 1;
            Some(mv.clone())
        } else {
            None
        }
    }


    pub fn next(&mut self, board: &Board) -> Option<Move> {
        match self.phase {
            MoveOrderingPhase::PvNode => self.next_pv_node(board),
            MoveOrderingPhase::WinningCapturesPromotions => self.next_winning_cap(board),
            MoveOrderingPhase::EqualCaptures => self.next_equal_cap(board),
            MoveOrderingPhase::Quiet => self.next_quiet(board),
            MoveOrderingPhase::LosingCaptures => self.next_non_filter()
        }
    }
}


enum QuiesceOrderingPhase {
    WinningCapturesPromotions,
    EqualCaptures,
    LosingCaptures
}

pub struct QuiesceOrderedMovesIter {
    moves: MoveList,
    phase: QuiesceOrderingPhase,
    curr_idx: usize,
}

impl QuiesceOrderedMovesIter  {
    pub fn new(moves: MoveList) -> Self {
        QuiesceOrderedMovesIter { moves, phase: QuiesceOrderingPhase::WinningCapturesPromotions, curr_idx: 0 }
    }

    pub fn next(&mut self, board: &Board) -> Option<Move> {
        match self.phase {
            QuiesceOrderingPhase::WinningCapturesPromotions => self.next_winning_cap(board),
            QuiesceOrderingPhase::EqualCaptures => self.next_equal_cap(board),
            QuiesceOrderingPhase::LosingCaptures => self.next_losing_cap()
        }
    }

    fn next_winning_cap(&mut self, board: &Board) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if mv.is_capture() && cap_score(mv, board) > 0 {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            }
            self.curr_idx += 1;
        }
        self.phase = QuiesceOrderingPhase::EqualCaptures;
        self.next(board)
    }


    fn next_equal_cap(&mut self, board: &Board) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if mv.is_capture() && cap_score(mv, board) == 0 {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            }
            self.curr_idx += 1;
        }
        self.phase = QuiesceOrderingPhase::LosingCaptures;
        self.next(board)
    }


    fn next_losing_cap(&mut self) -> Option<Move> {
        while self.curr_idx < self.moves.get_count() {
            let mv = self.moves.get_move(self.curr_idx);
            if mv.is_capture() {
                let res = mv.clone();
                self.moves.remove(self.curr_idx);
                return Some(res);
            }
            self.curr_idx += 1;
        }
        None
    }
}




fn cap_score(mv: &Move, board: &Board) -> i32 {
    if mv.is_en_passant() {
        0
    } else {
        let s_piece = board.get_ally_pieces().get_piece_at(mv.get_start_field());
        let t_piece = board.get_enemy_pieces().get_piece_at(mv.get_target_field());

        MIDGAME_PIECE_VALUES.values[t_piece] - MIDGAME_PIECE_VALUES.values[s_piece]
    }
}
