use crate::{position::board::Board, constants::*};

impl Board {
    pub fn to_fen(&self) -> String {
        let strings: Vec<String> = vec![
            self.parse_pieces(),
            self.parse_side_to_move(),
            self.parse_castling_rights(),
            self.parse_en_passant_file(),
            self.parse_halfmove_clock(),
            self.parse_move_clock(),
        ];
        strings.join(" ")
    }

    fn parse_pieces(&self) -> String {
        let white_piece_set = &self.players[WHITE];
        let black_piece_set = &self.players[BLACK];
        let mut rank_strings: Vec<String> = vec![];
        for rank in (0..8).rev() {
            let mut piece_str = String::new();
            let mut skip_count = 0;
            for file in 0..8 {
                let field = rank * 8 + file;
                let mut make_upper = true;
                let white_piece = white_piece_set.get_piece_at(field);
                let black_piece = black_piece_set.get_piece_at(field);
                let piece = if black_piece != NONE {
                    make_upper = false;
                    black_piece
                } else {
                    white_piece
                };
                if piece == NONE {
                    skip_count += 1;
                } else {
                    if skip_count != 0 {
                        let num_ch = char::from_digit(skip_count as u32, 10).unwrap();
                        piece_str.push(num_ch);
                        skip_count = 0;
                    }
                    let mut ch = piece_to_char(piece);
                    if make_upper {
                        ch = ch.to_ascii_uppercase();
                    }
                    piece_str.push(ch);
                }
            }
            if skip_count != 0 {
                let num_ch = char::from_digit(skip_count as u32, 10).unwrap();
                piece_str.push(num_ch);
            }
            rank_strings.push(piece_str);
        }
        rank_strings.join("/")
    }

    fn parse_side_to_move(&self) -> String {
        if self.us == WHITE {
            "w".to_string()
        } else {
            "b".to_string()
        }
    }

    fn parse_castling_rights(&self) -> String {
        let state = self.get_state();
        let castling_rights = state.get_castling_rights();
        if castling_rights == 0 {
            "-".to_string()
        } else {
            let mut castle_str = String::new();
            if state.can_castle_kingside(WHITE) {
                castle_str.push('K');
            }
            if state.can_castle_queenside(WHITE) {
                castle_str.push('Q');
            }
            if state.can_castle_kingside(BLACK) {
                castle_str.push('k');
            }
            if state.can_castle_queenside(BLACK) {
                castle_str.push('q');
            }
            castle_str
        }
    }

    fn parse_en_passant_file(&self) -> String {
        let file = self.get_state().get_en_passant_file();
        if file == 0 {
            "-".to_string()
        } else {
            let file = file - 1;
            let file_char = file_to_char(file as u32).to_string();
            if self.us == WHITE {
                file_char.to_string() + "6"
            } else {
                file_char.to_string() + "3"
            }
        }
    }

    fn parse_halfmove_clock(&self) -> String {
        let halfmove_clock = self.get_state().get_halfmove_clock();
        halfmove_clock.to_string()
    }

    fn parse_move_clock(&self) -> String {
        let move_clock = self.get_state().get_move_clock();
        move_clock.to_string()
    }
}

fn file_to_char(file: u32) -> char {
    if file > 7 {
        panic!("Invalid file number in file_to_char: {file}");
    }

    //97 is code for 'a'
    let code = 97 + file as u8;
    code as char
}

fn piece_to_char(piece: usize) -> char {
    match piece {
        PAWN => 'p',
        ROOK => 'r',
        BISHOP => 'b',
        KNIGHT => 'n',
        QUEEN => 'q',
        KING => 'k',
        _ => panic!("Invalid piece code in piece to char {piece}"),
    }
}

#[cfg(test)]
mod test {
    use crate::fen_parsing::parse_fen::parse_fen;

    #[test]
    fn should_parse_back_starting_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        should_parse_fen_and_back(fen);
    }

    #[test]
    fn should_parse_back_after_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        should_parse_fen_and_back(fen);
    }

    #[test]
    fn should_parse_back_in_middlegame() {
        let fen = "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 4";
        should_parse_fen_and_back(fen);
    }

    #[test]
    fn should_parse_back_in_endgame() {
        let fen = "8/8/8/8/8/8/4k3/4K2R w - - 0 1";
        should_parse_fen_and_back(fen);
    }

    #[test]
    fn should_parse_back_castling_rights() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/2KR1BNR b kq - 1 1";
        should_parse_fen_and_back(fen);
    }

    fn should_parse_fen_and_back(fen: &str) {
        let board = parse_fen(fen).unwrap();
        let result = board.to_fen();
        assert_eq!(fen, &result);
    }
}
