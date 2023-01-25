use crate::bitboard::Bitboard;
use crate::piece::{Piece, PieceType};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub fn value(&self) -> u32 {
        *self as u32
    }
    pub fn enemy(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[non_exhaustive]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Side,
    pub occupied_squares: Bitboard,
    pub side_bitboards: [Bitboard; 2],
    pub piece_bitboards: HashMap<Piece, Bitboard>,
}

impl Board {
    pub fn friendly_squares(&self) -> Bitboard {
        self.side_bitboards[self.side_to_move.value() as usize]
    }
    pub fn enemy_squares(&self) -> Bitboard {
        self.side_bitboards[self.side_to_move.enemy().value() as usize]
    }

    pub fn new() -> Board {
        let piece_bitboards = HashMap::from([
            (Piece::new(PieceType::Pawn, Side::White), Bitboard(0)),
            (Piece::new(PieceType::Knight, Side::White), Bitboard(0)),
            (Piece::new(PieceType::Bishop, Side::White), Bitboard(0)),
            (Piece::new(PieceType::Rook, Side::White), Bitboard(0)),
            (Piece::new(PieceType::Queen, Side::White), Bitboard(0)),
            (Piece::new(PieceType::King, Side::White), Bitboard(0)),
            (Piece::new(PieceType::Pawn, Side::Black), Bitboard(0)),
            (Piece::new(PieceType::Knight, Side::Black), Bitboard(0)),
            (Piece::new(PieceType::Bishop, Side::Black), Bitboard(0)),
            (Piece::new(PieceType::Rook, Side::Black), Bitboard(0)),
            (Piece::new(PieceType::Queen, Side::Black), Bitboard(0)),
            (Piece::new(PieceType::King, Side::Black), Bitboard(0)),
        ]);
        Board {
            squares: [Option::<Piece>::None; 64],
            side_to_move: Side::White,
            occupied_squares: Bitboard(0),
            side_bitboards: [Bitboard(0); 2],
            piece_bitboards,
        }
    }
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::new();
        board.load_fen(fen);
        board.set_bitboards_from_squares();
        board
    }

    pub fn load_fen(&mut self, fen: &str) {
        let piece_types = HashMap::from([
            ('p', PieceType::Pawn),
            ('n', PieceType::Knight),
            ('b', PieceType::Bishop),
            ('r', PieceType::Rook),
            ('q', PieceType::Queen),
            ('k', PieceType::King),
        ]);
        let ranks: Vec<&str> = fen.split('/').collect();
        for rank in 0..ranks.len() {
            let mut file = 0;
            for i in 0..ranks[rank].len() {
                let piece_char = ranks[rank].as_bytes()[i] as char;
                if piece_char.is_numeric() {
                    let piece_char: char = ranks[rank].as_bytes()[i] as char;
                    file += piece_char.to_digit(10).unwrap();
                } else {
                    let square = rank * 8 + file as usize;
                    if piece_char.is_uppercase() {
                        self.squares[square] = Some(Piece {
                            piece_type: piece_types
                                .get(&piece_char.to_ascii_lowercase())
                                .copied()
                                .unwrap(),
                            side: Side::White,
                        })
                    } else {
                        self.squares[square] = Some(Piece {
                            piece_type: piece_types.get(&piece_char).copied().unwrap(),
                            side: Side::Black,
                        })
                    }
                }
                file += 1;
            }
        }
        self.set_bitboards_from_squares();
    }
    pub fn set_squares_from_bitboards(&mut self) {
        for entry in self.piece_bitboards.iter_mut() {
            let mut bitboard = entry.1.clone();
            while bitboard != 0 {
                let square = bitboard.pop_lsb();
                self.squares[square as usize] = Some(*entry.0);
            }
        }
    }
    pub fn set_bitboards_from_squares(&mut self) {
        for square in 0..64u32 {
            if let Some(piece) = self.squares[square as usize] {
                self.piece_bitboards
                    .get_mut(&piece)
                    .unwrap()
                    .set_bit(&square);
                self.side_bitboards[piece.side.value() as usize].set_bit(&square);
                self.occupied_squares.set_bit(&square);
            } else {
                self.piece_bitboards
                    .values_mut()
                    .for_each(|x| x.clear_bit(&square));
                self.side_bitboards
                    .iter_mut()
                    .for_each(|x| x.clear_bit(&square));
                self.occupied_squares.clear_bit(&square);
            }
        }
    }
    pub fn make_move(&self) {}
    pub fn unmake_move(&self) {}
    pub fn update_board_state(&mut self) {}
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_chars = HashMap::from([
            (PieceType::Pawn, 'p'),
            (PieceType::Knight, 'n'),
            (PieceType::Bishop, 'b'),
            (PieceType::Rook, 'r'),
            (PieceType::Queen, 'q'),
            (PieceType::King, 'k'),
        ]);
        for rank in 0..8 {
            write!(f, "{}", rank + 1).unwrap();
            for file in 0..8 {
                write!(f, "{}", ' ').unwrap();
                match self.squares[rank * 8 + file] {
                    Some(piece) => {
                        let piece_char = piece_chars.get(&piece.piece_type).unwrap();
                        if piece.side == Side::White {
                            write!(f, "{}", piece_char.to_ascii_uppercase()).unwrap();
                        } else {
                            write!(f, "{}", piece_char).unwrap();
                        }
                    }
                    None => write!(f, "{}", '.').unwrap(),
                }
            }
            writeln!(f).unwrap();
        }
        write!(f, "{}", ' ').unwrap();
        for file in 'a'..='h' {
            write!(f, "{}", ' ').unwrap();
            write!(f, "{}", file).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_fen_sets_correct_squares() {
        let mut squares = [Option::<Piece>::None; 64];
        for i in 0..8 {
            squares[8 + i] = Some(Piece::new(PieceType::Pawn, Side::Black));
            squares[48 + i] = Some(Piece::new(PieceType::Pawn, Side::White));
        }
        squares[0] = Some(Piece::new(PieceType::Rook, Side::Black));
        squares[1] = Some(Piece::new(PieceType::Knight, Side::Black));
        squares[2] = Some(Piece::new(PieceType::Bishop, Side::Black));
        squares[3] = Some(Piece::new(PieceType::Queen, Side::Black));
        squares[4] = Some(Piece::new(PieceType::King, Side::Black));
        squares[5] = Some(Piece::new(PieceType::Bishop, Side::Black));
        squares[6] = Some(Piece::new(PieceType::Knight, Side::Black));
        squares[7] = Some(Piece::new(PieceType::Rook, Side::Black));

        squares[56] = Some(Piece::new(PieceType::Rook, Side::White));
        squares[57] = Some(Piece::new(PieceType::Knight, Side::White));
        squares[58] = Some(Piece::new(PieceType::Bishop, Side::White));
        squares[59] = Some(Piece::new(PieceType::Queen, Side::White));
        squares[60] = Some(Piece::new(PieceType::King, Side::White));
        squares[61] = Some(Piece::new(PieceType::Bishop, Side::White));
        squares[62] = Some(Piece::new(PieceType::Knight, Side::White));
        squares[63] = Some(Piece::new(PieceType::Rook, Side::White));

        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(squares, board.squares);
    }
    #[test]
    fn sets_correct_bitboards_from_squares() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        board.set_bitboards_from_squares();
        let white_pawn_bitboard = board
            .piece_bitboards
            .get(&Piece::new(PieceType::Pawn, Side::White))
            .unwrap();
        assert_eq!(white_pawn_bitboard.0, 0x00FF000000000000)
    }
}