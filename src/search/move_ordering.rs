use crate::{constants::PAWN, evaluation::piece_values::MIDGAME_PIECE_VALUES, moving::{move_list::MoveList, mv::Move}, position::board::Board, search::history::{self, HistoryTable}};

pub struct OrderedMovesIter {
    moves: Vec<ClassifiedMove>,
    phase: MoveOrderingPhase,
    pv_node: Option<Move>,
}

enum MoveOrderingPhase {
    PvNode,
    Promotions,
    WinningCaptures,
    EqualCaptures,
    LosingCaptures,
    Quiet,
    Exhausted
}

const V_FACTOR: i32 = 8; //this is how many times more valuable the victim is than the attacker

impl OrderedMovesIter {
    pub fn new(mut moves: MoveList, pv_node: Move, board: &Board, hist: &HistoryTable) -> Self {
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

        let mut move_kinds: Vec<ClassifiedMove> = Vec::with_capacity(moves.get_count());
        for mv in moves.iter() {
            let move_kind = classify_move(mv.clone(), board, hist);
            move_kinds.push(move_kind);
        }
        OrderedMovesIter { moves: move_kinds, phase: MoveOrderingPhase::PvNode, pv_node: pv_node_u }
    }

    fn next_pv_node(&mut self) -> Option<Move> {
        self.phase = MoveOrderingPhase::Promotions;
        if self.pv_node.is_some() {
            self.pv_node.clone()
        } else {
            self.next()
        }
    }

    fn next_by_kind(&mut self, kind: MoveKind, next_phase: MoveOrderingPhase) -> Option<Move> {
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
            self.next()
        }
    }

    pub fn next(&mut self) -> Option<Move> {
        match self.phase {
            MoveOrderingPhase::PvNode => self.next_pv_node(),
            MoveOrderingPhase::Promotions => self.next_by_kind(MoveKind::Promotion, MoveOrderingPhase::WinningCaptures),
            MoveOrderingPhase::WinningCaptures => self.next_by_kind(MoveKind::WinningCapture, MoveOrderingPhase::EqualCaptures),
            MoveOrderingPhase::EqualCaptures => self.next_by_kind(MoveKind::EqualCapture, MoveOrderingPhase::Quiet),
            MoveOrderingPhase::Quiet => self.next_by_kind(MoveKind::QuietMove, MoveOrderingPhase::LosingCaptures),
            MoveOrderingPhase::LosingCaptures => self.next_by_kind(MoveKind::LosingCapture, MoveOrderingPhase::Exhausted),
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
                let move_kind = classify_move(mv.clone(), board, history);
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
