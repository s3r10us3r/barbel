use crate::board::board::Board;
use crate::board::zobrist_hashing::ZobristHasher;
use crate::constants::*;

#[derive(Debug, PartialEq)]
pub enum FenError {
    InvalidCharacter { ch: char, position: usize },
    InvalidRankCount { found: usize },
    InvalidStructure { reason: String },
    InvalidFileCount { found: usize, rank: usize },
}

pub fn parse_fen(fen: &str) -> Result<Board, FenError> {
    let mut board = Board::new(ZobristHasher::new());
    let splits: Vec<&str> = fen.split(" ").collect();
    if splits.len() < 4 {
        return Err(FenError::InvalidStructure {
            reason: format!(
                "Invalid number of splits, expected 6, found {}",
                splits.len()
            ),
        });
    }
    board = parse_pieces(board, splits[0])?;
    board = parse_side_to_move(board, splits[1])?;
    board = parse_castling_rights(board, splits[2])?;
    board = parse_en_passant_file(board, splits[3])?;
    if splits.len() > 4 {
        board = parse_half_move_clock(board, splits[4])?;
    }
    if splits.len() > 5 {
        board = parse_move_clock(board, splits[5])?;
    }
    board.compute_occ_and_checkers();
    board.compute_hash();
    Ok(board)
}

fn parse_pieces(mut board: Board, piece_str: &str) -> Result<Board, FenError> {
    let ranks: Vec<&str> = piece_str.split("/").collect();
    if ranks.len() < 8 {
        return Err(FenError::InvalidRankCount { found: ranks.len() });
    }
    let mut pos = 0;
    for (i, rank) in ranks.iter().enumerate() {
        let mut file = 0;
        for ch in rank.chars() {
            if let Some(num) = ch.to_digit(10) {
                file += num as usize;
            } else {
                if file >= 8 {
                    return Err(FenError::InvalidStructure {
                        reason: format!("Too many pieces in rank {i}"),
                    });
                }
                let piece_set = if ch.is_uppercase() {
                    &mut board.players[WHITE]
                } else {
                    &mut board.players[BLACK]
                };
                let piece_type = fen_chr_to_piece_type(ch.to_ascii_lowercase())
                    .ok_or(FenError::InvalidCharacter { ch, position: pos })?;
                let field: usize = (7 - i) * 8 + file;
                piece_set.add_piece(field, piece_type);
                file += 1;
            }
            pos += 1;
        }
        if file != 8 {
            return Err(FenError::InvalidFileCount {
                found: file,
                rank: i,
            });
        }
    }
    Ok(board)
}

fn fen_chr_to_piece_type(ch: char) -> Option<usize> {
    match ch {
        'r' => Some(ROOK),
        'b' => Some(BISHOP),
        'k' => Some(KING),
        'q' => Some(QUEEN),
        'n' => Some(KNIGHT),
        'p' => Some(PAWN),
        _ => None,
    }
}

fn parse_side_to_move(mut board: Board, side_to_move: &str) -> Result<Board, FenError> {
    match side_to_move {
        "w" => {
            board.us = WHITE;
            board.enemy = BLACK;
        }
        "b" => {
            board.us = BLACK;
            board.enemy = WHITE;
        }
        _ => {
            return Err(FenError::InvalidStructure {
                reason: format!("Invalid side to move field {side_to_move}, expected 'w' or 'b'"),
            })
        }
    };
    Ok(board)
}

fn parse_en_passant_file(mut board: Board, en_passant_field: &str) -> Result<Board, FenError> {
    if en_passant_field == "-" {
        Ok(board)
    } else {
        if en_passant_field.len() != 2 {
            return Err(FenError::InvalidStructure {
                reason: format!("Invalid en-passant structure: {en_passant_field}"),
            });
        }
        let chars: Vec<char> = en_passant_field.chars().collect();
        let file = match chars[0] {
            'a' => Ok(1),
            'b' => Ok(2),
            'c' => Ok(3),
            'd' => Ok(4),
            'e' => Ok(5),
            'f' => Ok(6),
            'g' => Ok(7),
            'h' => Ok(8),
            _ => Err(FenError::InvalidStructure {
                reason: format!("Invalid en-passant structure: {en_passant_field}"),
            }),
        }?;
        board.get_mut_state().set_en_passant_file(file);
        Ok(board)
    }
}

fn parse_castling_rights(mut board: Board, castling_rights: &str) -> Result<Board, FenError> {
    if castling_rights == "-" {
        Ok(board)
    } else {
        let state = board.get_mut_state();
        for ch in castling_rights.chars() {
            match ch {
                'K' => state.set_castling_rights_for(WHITE, KING),
                'k' => state.set_castling_rights_for(BLACK, KING),
                'Q' => state.set_castling_rights_for(WHITE, QUEEN),
                'q' => state.set_castling_rights_for(BLACK, QUEEN),
                _ => {
                    return Err(FenError::InvalidStructure {
                        reason: format!("Invalid castling rights character: {ch}"),
                    })
                }
            }
        }
        Ok(board)
    }
}

fn parse_half_move_clock(mut board: Board, half_move_str: &str) -> Result<Board, FenError> {
    let half_moves: u32 = match half_move_str.parse() {
        Ok(num) => num,
        Err(_) => {
            return Err(FenError::InvalidStructure {
                reason: format!("Invalid halfmove field: {half_move_str}"),
            })
        }
    };
    board.get_mut_state().set_halfmove_clock(half_moves);
    Ok(board)
}

fn parse_move_clock(mut board: Board, move_str: &str) -> Result<Board, FenError> {
    let moves: u32 = match move_str.parse() {
        Ok(num) => num,
        Err(_) => {
            return Err(FenError::InvalidStructure {
                reason: format!("Invalid move field: {move_str}"),
            })
        }
    };
    board.get_mut_state().set_move_clock(moves);
    Ok(board)
}

#[cfg(test)]
mod test {
    use super::parse_fen;
    use super::{BLACK, WHITE};
    #[test]
    fn should_correctly_parse_starting_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = parse_fen(fen).unwrap();
        let white_pieces = &board.players[WHITE];
        assert_eq!(white_pieces.get_pawns(), 0xff00, "white pawns");
        assert_eq!(white_pieces.get_knights(), 0x42, "white knights");
        assert_eq!(white_pieces.get_diagonals(), 0x2c, "white diagonals");
        assert_eq!(white_pieces.get_orthogonals(), 0x89, "white orthogonals");
        assert_eq!(white_pieces.get_queens(), 0x8, "white queens");
        assert_eq!(white_pieces.get_king(), 0x10, "white king");

        let black_pieces = &board.players[BLACK];
        assert_eq!(black_pieces.get_pawns(), 0xff000000000000, "black pawns");
        assert_eq!(
            black_pieces.get_knights(),
            0x4200000000000000,
            "black knights"
        );
        assert_eq!(
            black_pieces.get_diagonals(),
            0x2c00000000000000,
            "black diagonals"
        );
        assert_eq!(
            black_pieces.get_orthogonals(),
            0x8900000000000000,
            "black orthogonals"
        );
        assert_eq!(black_pieces.get_queens(), 0x800000000000000, "black queens");
        assert_eq!(black_pieces.get_king(), 0x1000000000000000, "black king");

        let state = board.get_state();
        assert_eq!(board.us, WHITE, "side to move");
        assert_eq!(state.get_castling_rights(), 0xf, "castling rights");
        assert_eq!(state.get_en_passant_file(), 0, "en passant");
        assert_eq!(state.get_halfmove_clock(), 0, "halfmove clock");
        assert_eq!(state.get_move_clock(), 1, "move clock");
    }

    #[test]
    fn should_correctly_parse_fen_after_e4() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let board = parse_fen(fen).unwrap();
        let white_pieces = &board.players[WHITE];
        assert_eq!(white_pieces.get_pawns(), 0x1000ef00, "white pawns");
        assert_eq!(white_pieces.get_knights(), 0x42, "white knights");
        assert_eq!(white_pieces.get_diagonals(), 0x2c, "white diagonals");
        assert_eq!(white_pieces.get_orthogonals(), 0x89, "white orthogonals");
        assert_eq!(white_pieces.get_queens(), 0x8, "white queens");
        assert_eq!(white_pieces.get_king(), 0x10, "white king");

        let black_pieces = &board.players[BLACK];
        assert_eq!(black_pieces.get_pawns(), 0xff000000000000, "black pawns");
        assert_eq!(
            black_pieces.get_knights(),
            0x4200000000000000,
            "black knights"
        );
        assert_eq!(
            black_pieces.get_diagonals(),
            0x2c00000000000000,
            "black diagonals"
        );
        assert_eq!(
            black_pieces.get_orthogonals(),
            0x8900000000000000,
            "black orthogonals"
        );
        assert_eq!(black_pieces.get_queens(), 0x800000000000000, "black queens");
        assert_eq!(black_pieces.get_king(), 0x1000000000000000, "black king");

        let state = board.get_state();
        assert_eq!(board.us, BLACK, "side to move");
        assert_eq!(state.get_castling_rights(), 0xf, "castling rights");
        assert_eq!(state.get_en_passant_file(), 5, "en passant");
        assert_eq!(state.get_halfmove_clock(), 0, "halfmove clock");
        assert_eq!(state.get_move_clock(), 1, "move clock");
    }

    #[test]
    fn should_correctly_parse_fen_in_middlegame() {
        let fen = "r1bq1rk1/ppp2ppp/2n2n2/3pp3/1bPP4/2N1PN2/PP2BPPP/R1BQ1RK1 w - e6 4 9";
        let board = parse_fen(fen).unwrap();
        let white_pieces = &board.players[WHITE];
        assert_eq!(white_pieces.get_pawns(), 0xc10e300, "white pawns");
        assert_eq!(white_pieces.get_knights(), 0x240000, "white knights");
        assert_eq!(white_pieces.get_diagonals(), 0x100c, "white diagonals");
        assert_eq!(white_pieces.get_orthogonals(), 0x29, "white orthogonals");
        assert_eq!(white_pieces.get_queens(), 0x8, "white queens");
        assert_eq!(white_pieces.get_king(), 0x40, "white king");

        let black_pieces = &board.players[BLACK];
        assert_eq!(black_pieces.get_pawns(), 0xe7001800000000, "black pawns");
        assert_eq!(black_pieces.get_knights(), 0x240000000000, "black knights");
        assert_eq!(
            black_pieces.get_diagonals(),
            0xc00000002000000,
            "black diagonals"
        );
        assert_eq!(
            black_pieces.get_orthogonals(),
            0x2900000000000000,
            "black orthogonals"
        );
        assert_eq!(black_pieces.get_queens(), 0x800000000000000, "black queens");
        assert_eq!(black_pieces.get_king(), 0x4000000000000000, "black king");

        let state = board.get_state();

        assert_eq!(board.us, WHITE, "side to move");
        assert_eq!(state.get_castling_rights(), 0, "castling rights");
        assert_eq!(state.get_en_passant_file(), 5, "en passant");
        assert_eq!(state.get_halfmove_clock(), 4, "halfmove clock");
        assert_eq!(state.get_move_clock(), 9, "move clock");
    }

    #[test]
    fn should_correctly_parse_fen_in_middlegame2() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let board = parse_fen(fen).unwrap();
        let white_pieces = &board.players[WHITE];
        assert_eq!(white_pieces.get_pawns(), 0x800000000c700, "white pawns");
        assert_eq!(white_pieces.get_knights(), 0x1002, "white knights");
        assert_eq!(white_pieces.get_diagonals(), 0x400000c, "white diagonals");
        assert_eq!(white_pieces.get_orthogonals(), 0x89, "white orthogonals");
        assert_eq!(white_pieces.get_queens(), 0x8, "white queens");
        assert_eq!(white_pieces.get_king(), 0x10, "white king");

        let black_pieces = &board.players[BLACK];
        assert_eq!(black_pieces.get_pawns(), 0xe3040000000000, "black pawns");
        assert_eq!(
            black_pieces.get_knights(),
            0x200000000002000,
            "black knights"
        );
        assert_eq!(
            black_pieces.get_diagonals(),
            0xc10000000000000,
            "black diagonals"
        );
        assert_eq!(
            black_pieces.get_orthogonals(),
            0x8900000000000000,
            "black orthogonals"
        );
        assert_eq!(black_pieces.get_queens(), 0x800000000000000, "black queens");
        assert_eq!(black_pieces.get_king(), 0x2000000000000000, "black king");

        let state = board.get_state();
        assert_eq!(board.us, WHITE, "side to move");
        assert_eq!(state.get_castling_rights(), 0xc, "castling rights");
        assert_eq!(state.get_en_passant_file(), 0, "en passant");
        assert_eq!(state.get_halfmove_clock(), 1, "halfmove clock");
        assert_eq!(state.get_move_clock(), 8, "move clock");
    }

    #[test]
    fn should_err_when_fen_has_too_little_ranks() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_err());
    }

    #[test]
    fn should_err_when_fen_has_too_many_fields_in_a_rank() {
        let fen = "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_err());
    }

    #[test]
    fn should_err_when_fen_has_too_little_fields_in_a_rank() {
        let fen = "rnbqkbnr/ppp1ppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert!(parse_fen(fen).is_err());
    }

    #[test]
    fn should_throw_when_the_side_to_move_is_invalid() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert!(parse_fen(fen).is_err());
    }
}
