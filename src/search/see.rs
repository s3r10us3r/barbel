use crate::{bitboard_helpers::flip_color, constants::{BISHOP, BLACK, KING, KNIGHT, NONE, PAWN, QUEEN, ROOK, WHITE}, evaluation::piece_values::MIDGAME_PIECE_VALUES, moving::{move_generation::MoveGenerator, mv::Move}, position::{board::Board, piece_set::PieceSet}};

pub fn see(board: &Board, color: usize, mv: &Move) {
    let moving = board.get_pieces(color);
    let staying = board.get_pieces(flip_color(color));

    let target = mv.get_target_field();

    //let moving = get_direct_attackers(moving, board.get_occupancy(), target, color, &board.mg);
    //let staying = get_direct_attackers(staying, board.get_occupancy(), target, flip_color(color), &board.mg);
}

struct SEEvaluator<'a> {
    pieces: [[u64; 6]; 2],
    piece_sets: [&'a PieceSet; 2],
    on_move: usize,
    used: u64,
}

struct SEESide<'a> {
    piece_set: &'a PieceSet,
    color: usize,
    curr_pieces: u64,
    curr_piece_type: usize,
    target: usize
}
/*
impl<'a> SEESide<'a> {
    fn new(piece_set: &'a PieceSet, color: usize, target: usize, board: &Board) -> Self {
        let pawn_attackers = get_pawn_attacks(target, color);
        SEESide { piece_set, color, curr_pieces: pawn_attackers, curr_piece_type: PAWN, target }
    }

    fn next_piece_type(&mut self, board: &Board, excluded: u64) {
        match self.curr_piece_type {
            PAWN => {
                self.curr_piece_type = KNIGHT;
                self.curr_pieces = board.mg.get_knight_attacks(self.target) & self.piece_set.get_knights() & !excluded;
            },
            KNIGHT => {
                self.curr_piece_type = BISHOP;
                self.curr_pieces = board.mg.get_bishop_attacks(self.target, board.get_occupancy() & !excluded) & self.piece_set.get_bishops() & !excluded;
            },
            BISHOP => {
                self.curr_piece_type = ROOK;
                self.curr_pieces = board.mg.get_rook_attacks(self.target, board.get_occupancy() & !excluded) & self.piece_set.get_rooks() & !excluded;
            },
            ROOK => {
                self.curr_piece_type = QUEEN;
                let occ = board.get_occupancy() & !excluded;
                self.curr_pieces = (board.mg.get_rook_attacks(self.target, occ) | board.mg.get_bishop_attacks(self.target, occ)) & self.piece_set.get_queens() & !excluded;
            },
            QUEEN => {
                self.curr_piece_type = KING;
                self.curr_pieces = board.mg.get_king_attacks(self.target) & self.piece_set.get_king() & !excluded;
            },
            KING => {
                self.curr_piece_type = NONE;
            }
            _ => {
                panic!("Invalid piece type in SEESide::next_piece_type");
            }
        }
    }
}

fn get_direct_attackers(pieces: &PieceSet, occ: u64, target: usize, color: usize, mg: &MoveGenerator) -> [u64; 6] {
    let pawn_attackers = mg.get_pawn_attacks(target, flip_color(color)) & pieces.get_pawns();
    let knight_attackers = mg.get_knight_attacks(target) & pieces.get_knights();
    let bishop_attackers = mg.get_bishop_attacks(target, occ) & pieces.get_bishops();
    let rook_attackers = mg.get_rook_attacks(target, occ) & pieces.get_rooks();
    let queen_attackers = (mg.get_bishop_attacks(target, occ) | mg.get_rook_attacks(target, occ)) & pieces.get_queens();
    let king_attack = mg.get_king_attacks(target) & pieces.get_king();

    [pawn_attackers, knight_attackers, bishop_attackers, rook_attackers, queen_attackers, king_attack]
}
*/
