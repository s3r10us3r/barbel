use crate::bitboard_helpers::{flip_color, isolate_lsb};
use crate::constants::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE};
use crate::evaluation::piece_values::MIDGAME_PIECE_VALUES;
use crate::moving::move_generation::{generate_moves, get_mg, is_legal};
use crate::moving::move_list::MoveList;
use crate::moving::mv::Move;
use crate::position::board::Board;
use crate::position::piece_set::PieceSet;
use crate::search::history::HistoryTable;
use crate::search::killers::KillerTable;

pub struct OrderedMovesIter {
    moves: Vec<ClassifiedMove>,
    phase: MoveOrderingPhase,
    pv_node: Move,
    ply: i32,
}

enum MoveOrderingPhase {
    PvNode,
    Generation,
    Promotions,
    WinningCaptures,
    KillerMoves,
    EqualCaptures,
    Quiet,
    LosingCaptures,
    Exhausted
}


impl OrderedMovesIter {
    pub fn new(pv_node: Move, ply: i32) -> Self {
        OrderedMovesIter { moves: Vec::new(), phase: MoveOrderingPhase::PvNode, pv_node, ply }
    }

    fn next_pv_node(&mut self, board: &Board, hist: &HistoryTable, killers: &KillerTable) -> Option<Move> {
        self.phase = MoveOrderingPhase::Generation;
        if !self.pv_node.is_null() && is_legal(&self.pv_node, board) {
            Some(self.pv_node)
        } else {
            self.next(board, hist, killers)
        }
    }

    fn generate_moves(&mut self, board: &Board, hist: &HistoryTable, killers: &KillerTable) -> Option<Move> {
        let mvs = generate_moves(board);
        let mut classified_moves: Vec<ClassifiedMove> = Vec::with_capacity(mvs.get_count());
        for mv in mvs.iter() {
            if *mv != self.pv_node {
                classified_moves.push(classify_move(*mv, board, hist, killers, self.ply));
            }
        }
        self.moves = classified_moves;
        self.phase = MoveOrderingPhase::Promotions;
        self.next(board, hist, killers)
    }

    fn next_by_kind(&mut self, board: &Board, hist: &HistoryTable, killers: &KillerTable, kind: MoveKind, next_phase: MoveOrderingPhase) -> Option<Move> {
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
            self.next(board, hist, killers)
        }
    }

    pub fn next(&mut self, board: &Board, hist: &HistoryTable, killers: &KillerTable) -> Option<Move> {
        match self.phase {
            MoveOrderingPhase::PvNode => self.next_pv_node(board, hist, killers),
            MoveOrderingPhase::Generation => self.generate_moves(board, hist, killers),
            MoveOrderingPhase::Promotions => self.next_by_kind(board, hist, killers, MoveKind::Promotion, MoveOrderingPhase::WinningCaptures),
            MoveOrderingPhase::WinningCaptures => self.next_by_kind(board, hist, killers, MoveKind::WinningCapture, MoveOrderingPhase::KillerMoves),
            MoveOrderingPhase::KillerMoves => self.next_by_kind(board, hist, killers, MoveKind::KillerMove, MoveOrderingPhase::EqualCaptures),
            MoveOrderingPhase::EqualCaptures => self.next_by_kind(board, hist, killers, MoveKind::EqualCapture, MoveOrderingPhase::Quiet),
            MoveOrderingPhase::Quiet => self.next_by_kind(board, hist,  killers, MoveKind::QuietMove, MoveOrderingPhase::LosingCaptures),
            MoveOrderingPhase::LosingCaptures => self.next_by_kind(board, hist, killers, MoveKind::LosingCapture, MoveOrderingPhase::Exhausted),
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
    pub fn new(moves: MoveList, board: &Board, history: &HistoryTable, killers: &KillerTable, ply: i32) -> Self {
        let mut move_kinds: Vec<ClassifiedMove> = Vec::with_capacity(moves.get_count());
        for mv in moves.iter() {
            if mv.is_promotion() || mv.is_capture() {
                let move_kind = classify_move(*mv, board, history, killers, ply);
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


fn classify_move(mv: Move, board: &Board, history: &HistoryTable, killers: &KillerTable, ply: i32) -> ClassifiedMove {
    if mv.is_promotion() {
        let mut score = MIDGAME_PIECE_VALUES.values[mv.get_promotion_piece()];
        if mv.is_capture() {
            score += see(board, &mv, board.us);
        }
        ClassifiedMove::new(MoveKind::Promotion, score, mv)
    }
    else if mv.is_capture() {
        let score = see(board, &mv, board.us);
        if score > 0 {
           ClassifiedMove::new(MoveKind::WinningCapture, score, mv)
        } else if score == 0 {
            ClassifiedMove::new(MoveKind::EqualCapture, score, mv)
        } else {
            ClassifiedMove::new(MoveKind::LosingCapture , score, mv)
        }
    } else if killers.is_killer(ply, &mv) {
        ClassifiedMove::new(MoveKind::KillerMove, 0, mv)
    } else {
        let hist_score = history.get_val(&mv);
        ClassifiedMove::new(MoveKind::QuietMove, hist_score, mv)
    }
}

#[derive(PartialEq)]
enum MoveKind {
    Promotion,
    WinningCapture,
    LosingCapture,
    EqualCapture,
    QuietMove,
    KillerMove
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

//king score is intentionally high
const SEE_PIECE_VALS: [i32; 6] = [100, 300, 300, 500, 900, 20_000];

fn see(board: &Board, cap: &Move, color: usize) -> i32 {
    let pieces = board.get_pieces(color);
    let enemy_pieces = board.get_pieces(flip_color(color));

    let target = cap.get_target_field();
    let start = cap.get_start_field();
    let mut piece_type = pieces.get_piece_at(start);
    let victim_type = if cap.is_en_passant() {PAWN} else {enemy_pieces.get_piece_at(target)};


    let mut gain = [0; 32];
    gain[0] = SEE_PIECE_VALS[victim_type];
    let mut d = 0;

    let mut attackers = [[0u64; 6]; 2];
    let mut on_attack = flip_color(color);
    let mut used = cap.get_start_bb();
    let mut occ = board.get_occupancy() & !used;
    if cap.is_en_passant() {
        let captured_pawn_sq = if color == WHITE { target - 8 } else { target + 8 };
        occ &= !(1u64 << captured_pawn_sq);
    }

    find_static_attackers(&mut attackers[color], board.get_pieces(color), target, color, used);
    find_static_attackers(&mut attackers[on_attack], board.get_pieces(on_attack), target, on_attack, used);

    loop {
        d += 1;
        gain[d] = SEE_PIECE_VALS[piece_type] - gain[d - 1];

        if -gain[d-1] < 0 && gain[d] < 0 {
            break;
        }
        let lva_res = get_lva(
            &mut attackers[on_attack], 
            target,
            occ, 
            board.get_pieces(on_attack), used
        );

        match lva_res {
            None => break,
            Some(p_type) => {
                piece_type = p_type;
                let bb = isolate_lsb(attackers[on_attack][piece_type]);
                attackers[on_attack][piece_type] &= !bb;
                used |= bb;
                occ &= !bb;
                on_attack = flip_color(on_attack);
            }
        }
    }
    while d > 1 {
        d -= 1;
        gain[d-1] = -std::cmp::max(-gain[d-1], gain[d]);
    }

    gain[0]
}

fn get_lva(attackers: &mut [u64; 6], sq: usize, occ: u64, pieces: &PieceSet, used: u64) -> Option<usize> {
    if attackers[PAWN] != 0 {
        return Some(PAWN);
    } 
    if attackers[KNIGHT] != 0 {
        return Some(KNIGHT);
    } 
    update_slider_attacks(attackers, sq, occ, pieces, used);
    if attackers[BISHOP] != 0 {
        return Some(BISHOP);
    } 
    if attackers[ROOK] != 0 {
        return Some(ROOK);
    } 
    if attackers[QUEEN] != 0 {
        return Some(QUEEN);
    } 
    if attackers[KING] != 0 {
        return Some(KING);
    } 
    None
}

fn update_slider_attacks(attackers: &mut [u64; 6], sq: usize, occ: u64, pieces: &PieceSet, used: u64) {
    let mg = get_mg();
    let rook_attacks = mg.get_rook_attacks(sq, occ);
    let bishop_attacks = mg.get_bishop_attacks(sq, occ);

    let bishop_attackers = pieces.get_bishops() & bishop_attacks & !used;
    attackers[BISHOP] = bishop_attackers;

    let rook_attackers = pieces.get_rooks() & rook_attacks & !used;
    attackers[ROOK] = rook_attackers;

    let queen_attackers = pieces.get_queens() & (rook_attacks | bishop_attacks) & !used; 
    attackers[QUEEN] = queen_attackers;
}

fn find_static_attackers(attackers: &mut [u64; 6], pieces: &PieceSet, sq: usize, color: usize, used: u64) {
    let mg = get_mg();

    let pawns = pieces.get_pawns();
    let pawn_attacks = mg.get_pawn_attacks(sq, flip_color(color));
    let pawn_attackers = pawns & pawn_attacks & !used;
    attackers[PAWN] = pawn_attackers;

    let knights = pieces.get_knights();
    let knight_attacks = mg.get_knight_attacks(sq);
    let knight_attackers = knights & knight_attacks & !used;
    attackers[KNIGHT] = knight_attackers;

    let king = pieces.get_king();
    let king_attacks = mg.get_king_attacks(sq);
    let king_attackers = king & king_attacks & !used;
    attackers[KING] = king_attackers;
}

