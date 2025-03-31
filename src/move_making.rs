use crate::board::*;
use crate::constants::*;
use crate::mv::*;

impl Board {
    pub fn make_move(&mut self, mv: &Move) {
        let state = self.get_state();
        let mut new_state = state.clone();
        new_state.clear_en_passant_file();
        new_state.set_captured_piece(NONE);
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        let moving_piece = self.players[self.us].get_piece_at(start);

        self.players[self.us].move_piece(start, target);
        let captured_piece = self.players[self.enemy].get_piece_at(target);
        self.players[self.enemy].take(target);
        new_state.set_captured_piece(captured_piece);

        if mv.is_double_pawn_move() {
            let file = get_file(target) + 1;
            new_state.set_en_passant_file(file as u32);
        } else if mv.is_en_passant() {
            let en_passant_field = if self.us == WHITE {
                target - 8
            } else {
                target + 8
            };
            self.players[self.enemy].take(en_passant_field);
            new_state.set_captured_piece(captured_piece);
        }

        if moving_piece == KING {
            new_state.disable_all_castling_rights(self.us);
        }
        if start == 0 || target == 0 {
            new_state.disable_queenside_castling_rights(WHITE);
        } else if start == 7 || target == 7 {
            new_state.disable_kingside_castling_rights(WHITE);
        } else if start == 56 || target == 56 {
            new_state.disable_queenside_castling_rights(BLACK);
        } else if start == 63 || target == 63 {
            new_state.disable_kingside_castling_rights(BLACK);
        }

        if mv.is_promotion() {
            let promotion_piece = mv.get_promotion_piece();
            self.players[self.us].take(target);
            self.players[self.us].add_piece(target, promotion_piece);
        }

        if mv.is_queenside_castle() {
            let rook_start = target - 2;
            let rook_target = start - 1;
            self.players[self.us].move_piece(rook_start, rook_target);
        } else if mv.is_kingside_castle() {
            let rook_start = target + 1;
            let rook_target = start + 1;
            self.players[self.us].move_piece(rook_start, rook_target);
        }

        if moving_piece == PAWN || mv.is_capture() {
            new_state.clear_halfmove_clock();
        } else {
            new_state.increment_halfmove_clock();
        }

        if self.us == BLACK {
            new_state.increase_move_clock();
        }
        self.push_state_stack(new_state);
        (self.us, self.enemy) = (self.enemy, self.us);
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        (self.us, self.enemy) = (self.enemy, self.us);
        let start = mv.get_start_field();
        let target = mv.get_target_field();
        let popped_state = self.pop_state();
        if mv.is_en_passant() {
            self.players[self.us].move_piece(target, start);
            let en_passant_square = if self.us == WHITE {
                target - 8
            } else {
                target + 8
            };
            self.players[self.enemy].add_piece(en_passant_square, PAWN);
            return;
        } else if mv.is_queenside_castle() {
            self.players[self.us].unmake_queenside_castle();
        } else if mv.is_kingside_castle() {
            self.players[self.us].unmake_kingside_castle();
        } else {
            if mv.is_capture() {
                let piece = popped_state.get_captured_piece();
                self.players[self.enemy].add_piece(target, piece as u8);
            }
            if mv.is_promotion() {
                self.players[self.us].take(target);
                self.players[self.us].add_piece(start, PAWN);
            } else {
                self.players[self.us].move_piece(target, start);
            }
        }
    }
}

fn construct_move(start: &str, target: &str, code: u16) -> Move {
    let start = field_str_to_num(start);
    let target = field_str_to_num(target);
    Move::new(start, target, code)
}

fn field_str_to_num(field_str: &str) -> u16 {
    let chars: Vec<char> = field_str.chars().collect();
    let file = chars[0] as u16 - 'a' as u16;
    let rank = (chars[1].to_digit(10).unwrap() - 1) as u16;
    rank * 8 + file
}

#[inline]
fn get_file(field: u16) -> u16 {
    field % 8
}

#[cfg(test)]
mod test {
    use super::{construct_move, Move};
    use crate::fen_parsing::parse_fen;
    #[test]
    fn should_make_and_unmake_e2_to_e4() {
        let fen_before = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let mv = construct_move("e2", "e4", 1);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_d7_to_d5() {
        let fen_before = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 1 1";
        let fen_after = "rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR w KQkq d6 0 2";
        let mv = construct_move("d7", "d5", 1);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_captured() {
        let fen_before = "4k3/8/8/4p3/8/5N2/8/4K3 w - - 0 1";
        let fen_after = "4k3/8/8/4N3/8/8/8/4K3 b - - 0 1";
        let mv = construct_move("f3", "e5", 0b0100);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_en_passant() {
        let fen_before = "r1bqkbnr/ppp1pppp/2n5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let fen_after = "r1bqkbnr/ppp1pppp/2nP4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 3";
        let mv = construct_move("e5", "d6", 0b0101);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_white_queenside_castle() {
        let fen_before = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR b kq - 1 1";
        let mv = construct_move("e1", "c1", 3);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_white_kingside_castle() {
        let fen_before = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1";
        let fen_after = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 b kq - 1 1";
        let mv = construct_move("e1", "g1", 2);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_black_queenside_castle() {
        let fen_before = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let fen_after = "2kr1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 1 2";
        let mv = construct_move("e8", "c8", 3);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_black_kingside_castle() {
        let fen_before = "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        let fen_after = "rnbq1rk1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 1 2";
        let mv = construct_move("e8", "g8", 2);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotion_capture_to_queen() {
        let fen_before = "rnbqkbnr/ppppppPp/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnQ/pppppp1p/8/8/8/8/PPPPP1PP/RNBQKBNR b KQq - 0 1";
        let mv = construct_move("g7", "h8", 15);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotion_capture_to_knight() {
        let fen_before = "rnbqkbnr/ppppppPp/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnN/pppppp1p/8/8/8/8/PPPPP1PP/RNBQKBNR b KQq - 0 1";
        let mv = construct_move("g7", "h8", 12);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotion_capture_to_bishop() {
        let fen_before = "rnbqkbnr/ppppppPp/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnB/pppppp1p/8/8/8/8/PPPPP1PP/RNBQKBNR b KQq - 0 1";
        let mv = construct_move("g7", "h8", 13);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotion_capture_to_rook() {
        let fen_before = "rnbqkbnr/ppppppPp/8/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 1";
        let fen_after = "rnbqkbnR/pppppp1p/8/8/8/8/PPPPP1PP/RNBQKBNR b KQq - 0 1";
        let mv = construct_move("g7", "h8", 14);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotione_to_queen() {
        let fen_before = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1";
        let fen_after = "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1";
        let mv = construct_move("b7", "b8", 11);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotione_to_rook() {
        let fen_before = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1";
        let fen_after = "1R2k3/8/8/8/8/8/8/4K3 b - - 0 1";
        let mv = construct_move("b7", "b8", 10);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotione_to_bishop() {
        let fen_before = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1";
        let fen_after = "1B2k3/8/8/8/8/8/8/4K3 b - - 0 1";
        let mv = construct_move("b7", "b8", 9);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    #[test]
    fn should_make_and_unmake_when_promotione_to_knight() {
        let fen_before = "4k3/1P6/8/8/8/8/8/4K3 w - - 0 1";
        let fen_after = "1N2k3/8/8/8/8/8/8/4K3 b - - 0 1";
        let mv = construct_move("b7", "b8", 8);
        should_make_and_unmake(fen_before, fen_after, mv);
    }

    fn should_make_and_unmake(fen_before: &str, fen_after: &str, mv: Move) {
        let mut board = parse_fen(fen_before).unwrap();
        board.make_move(&mv);
        let result_fen = board.to_fen();
        assert_eq!(fen_after, &result_fen, "make");
        board.unmake_move(&mv);
        let result_fen = board.to_fen();
        assert_eq!(fen_before, &result_fen, "unmake");
    }
}
