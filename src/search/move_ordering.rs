use crate::{constants::PAWN, evaluation::piece_values::MIDGAME_PIECE_VALUES, moving::{move_list::MoveList, mv::Move}, position::board::Board, search::history::{HistoryTable}};

pub struct OrderedMovesIter {
    moves: Vec<ClassifiedMove>,
    phase: MoveOrderingPhase,
    pv_node: Move,
}

enum MoveOrderingPhase {
    PvNode,
    Generation,
    Promotions,
    WinningCaptures,
    EqualCaptures,
    LosingCaptures,
    Quiet,
    Exhausted
}

const V_FACTOR: i32 = 8; //this is how many times more valuable the victim is than the attacker

impl OrderedMovesIter {
    pub fn new(pv_node: Move) -> Self {
        OrderedMovesIter { moves: Vec::new(), phase: MoveOrderingPhase::PvNode, pv_node }
    }

    fn next_pv_node(&mut self, board: &Board, hist: &HistoryTable) -> Option<Move> {
        self.phase = MoveOrderingPhase::Generation;
        if !self.pv_node.is_null() && board.is_legal(&self.pv_node) {
            Some(self.pv_node)
        } else {
            self.next(board, hist)
        }
    }

    fn generate_moves(&mut self, board: &Board, hist: &HistoryTable) -> Option<Move> {
        let mvs = board.mg.generate_moves(board);
        let mut classified_moves: Vec<ClassifiedMove> = Vec::with_capacity(mvs.get_count());
        for mv in mvs.iter() {
            if *mv != self.pv_node {
                classified_moves.push(classify_move(*mv, board, hist));
            }
        }
        self.moves = classified_moves;
        self.phase = MoveOrderingPhase::Promotions;
        self.next(board, hist)
    }

    fn next_by_kind(&mut self, board: &Board,  hist: &HistoryTable, kind: MoveKind, next_phase: MoveOrderingPhase) -> Option<Move> {
        let mut best_i = 0;
        let mut best_score = i32::MIN; 

        for i in 0..self.moves.len() {
            let mv = &self.moves[i];
            if mv.kind == kind && mv.score > best_score {
                best_score = mv.score;
                best_i = i;
            }
        }

        if best_score != i32::MIN {
            let mv = self.moves.swap_remove(best_i);
            Some(mv.mv)
        } else {
            self.phase = next_phase;
            self.next(board, hist)
        }
    }

    pub fn next(&mut self, board: &Board, hist: &HistoryTable) -> Option<Move> {
        match self.phase {
            MoveOrderingPhase::PvNode => self.next_pv_node(board, hist),
            MoveOrderingPhase::Generation => self.generate_moves(board, hist),
            MoveOrderingPhase::Promotions => self.next_by_kind(board, hist, MoveKind::Promotion, MoveOrderingPhase::WinningCaptures),
            MoveOrderingPhase::WinningCaptures => self.next_by_kind(board, hist, MoveKind::WinningCapture, MoveOrderingPhase::EqualCaptures),
            MoveOrderingPhase::EqualCaptures => self.next_by_kind(board, hist, MoveKind::EqualCapture, MoveOrderingPhase::Quiet),
            MoveOrderingPhase::Quiet => self.next_by_kind(board, hist, MoveKind::QuietMove, MoveOrderingPhase::LosingCaptures),
            MoveOrderingPhase::LosingCaptures => self.next_by_kind(board, hist, MoveKind::LosingCapture, MoveOrderingPhase::Exhausted),
            MoveOrderingPhase::Exhausted => None
        }
    }
}


enum QuiesceOrderingPhase {
    Promotions,
    WinningCaptures,
    EqualCaptures,
    LosingCaptures,
    Exhausted
}

pub struct QuiesceOrderedMovesIter {
    moves: Vec<ClassifiedMove>,
    phase: QuiesceOrderingPhase,
}

impl QuiesceOrderedMovesIter  {
    pub fn new(moves: MoveList, board: &Board, history: &HistoryTable) -> Self {
        let mut move_kinds: Vec<ClassifiedMove> = Vec::with_capacity(moves.get_count());
        for mv in moves.iter() {
            if mv.is_promotion() || mv.is_capture() {
                let move_kind = classify_move(*mv, board, history);
                move_kinds.push(move_kind);
            }
        }
        QuiesceOrderedMovesIter { moves: move_kinds, phase: QuiesceOrderingPhase::Promotions }
    }

    pub fn next(&mut self) -> Option<Move> {
        match self.phase {
            QuiesceOrderingPhase::Promotions => self.next_by_kind(MoveKind::Promotion, QuiesceOrderingPhase::WinningCaptures),
            QuiesceOrderingPhase::WinningCaptures => self.next_by_kind(MoveKind::WinningCapture, QuiesceOrderingPhase::EqualCaptures),
            QuiesceOrderingPhase::EqualCaptures => self.next_by_kind(MoveKind::EqualCapture, QuiesceOrderingPhase::LosingCaptures),
            QuiesceOrderingPhase::LosingCaptures => self.next_by_kind(MoveKind::LosingCapture, QuiesceOrderingPhase::Exhausted),
            QuiesceOrderingPhase::Exhausted => None
        }
    }
    
    
    fn next_by_kind(&mut self, kind: MoveKind, next_phase: QuiesceOrderingPhase) -> Option<Move> {
        let mut best_i = 0;
        let mut best_score = i32::MIN; 

        for i in 0..self.moves.len() {
            let mv = &self.moves[i];
            if mv.kind == kind && mv.score > best_score{
                best_score = mv.score;
                best_i = i;
            }
        }

        if best_score != i32::MIN {
            let mv = self.moves.swap_remove(best_i);
            Some(mv.mv)
        } else {
            self.phase = next_phase;
            self.next()
        }
    }
}

fn cap_score_raw(aggressor: usize, victim: usize) -> i32 {
    MIDGAME_PIECE_VALUES.values[victim] - MIDGAME_PIECE_VALUES.values[aggressor]
}


fn mvv_lva(aggressor: usize, victim: usize) -> i32 {
    MIDGAME_PIECE_VALUES.values[victim] * V_FACTOR - MIDGAME_PIECE_VALUES.values[aggressor]
}


fn classify_move(mv: Move, board: &Board, history: &HistoryTable) -> ClassifiedMove {
    if mv.is_promotion() {
        let mut score = MIDGAME_PIECE_VALUES.values[mv.get_promotion_piece()];
        if mv.is_capture() {
            let victim = board.get_enemy_pieces().get_piece_at(mv.get_target_field());
            score += MIDGAME_PIECE_VALUES.values[victim] * V_FACTOR - MIDGAME_PIECE_VALUES.values[PAWN];
        }
        ClassifiedMove::new(MoveKind::Promotion, score, mv)
    }
    else if mv.is_capture() {
        let aggressor = board.get_ally_pieces().get_piece_at(mv.get_start_field());
        let victim = if mv.is_en_passant() {
            PAWN
        } else {
            board.get_enemy_pieces().get_piece_at(mv.get_target_field())
        };

        let raw_score = cap_score_raw(aggressor, victim);
        if raw_score > 0 {
           ClassifiedMove::new(MoveKind::WinningCapture, mvv_lva(aggressor, victim), mv)
        } else if raw_score == 0 {
            ClassifiedMove::new(MoveKind::EqualCapture,mvv_lva(aggressor, victim), mv)
        } else {
            ClassifiedMove::new(MoveKind::LosingCapture ,mvv_lva(aggressor, victim), mv)
        }
    } else {
        ClassifiedMove::new(MoveKind::QuietMove, history.get_val(&mv), mv)
    }
}

#[derive(PartialEq)]
enum MoveKind {
    Promotion,
    WinningCapture,
    LosingCapture,
    EqualCapture,
    QuietMove,
}

struct ClassifiedMove {
    kind: MoveKind,
    score: i32,
    mv: Move
}

impl ClassifiedMove {
    fn new(kind: MoveKind, score: i32, mv: Move) -> Self {
        ClassifiedMove { kind, score, mv }
    }
}
