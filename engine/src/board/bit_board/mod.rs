use std::fmt::Debug;

use crate::piece::{position::Position, ChessPiece, Color, Type};

/*
8| 56  57  58  59  60  61  62  63
7| 48  49  50  51  52  53  54  55
6| 40  41  42  43  44  45  46  47
5| 32  33  34  35  36  37  38  39
4| 24  25  26  27  28  29  30  31
3| 16  17  18  19  20  21  22  23
2| 8   9   10  11  12  13  14  15
1| 0   1   2   3   4   5   6   7
 ------------------------------
  a   b   c   d   e   f   g   h

*/
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BitBoard {
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,
    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let white_pawns = format!("{:064b}", self.white_pawns);
        let white_knights = format!("{:064b}", self.white_knights);
        let white_bishops = format!("{:064b}", self.white_bishops);
        let white_rooks = format!("{:064b}", self.white_rooks);
        let white_queens = format!("{:064b}", self.white_queens);
        let white_king = format!("{:064b}", self.white_king);

        let black_pawns = format!("{:064b}", self.black_pawns);
        let black_knights = format!("{:064b}", self.black_knights);
        let black_bishops = format!("{:064b}", self.black_bishops);
        let black_rooks = format!("{:064b}", self.black_rooks);
        let black_queens = format!("{:064b}", self.black_queens);
        let black_king = format!("{:064b}", self.black_king);

        f.debug_struct("BitBoard")
            .field("white_pawns", &white_pawns)
            .field("white_knights", &white_knights)
            .field("white_bishops", &white_bishops)
            .field("white_rooks", &white_rooks)
            .field("white_queens", &white_queens)
            .field("white_king", &white_king)
            .field("black_pawns", &black_pawns)
            .field("black_knights", &black_knights)
            .field("black_bishops", &black_bishops)
            .field("black_rooks", &black_rooks)
            .field("black_queens", &black_queens)
            .field("black_king", &black_king)
            .finish()
    }
}

impl BitBoard {
    pub fn new() -> Self {
        //8765_4321
        //hgfe_dcba
        let white_rooks: u64 = 0b1000_0001;
        let white_knights: u64 = 0b0100_0010;
        let white_bishops: u64 = 0b0010_0100;
        let white_queens: u64 = 0b0000_1000;
        let white_king: u64 = 0b0001_0000;
        let white_pawns: u64 = 0b1111_1111_0000_0000;

        let black_rooks: u64 = 0b10000001 << 56;
        let black_knights: u64 = 0b01000010 << 56;
        let black_bishops: u64 = 0b00100100 << 56;
        let black_queens: u64 = 0b0000_1000 << 56;
        let black_king: u64 = 0b0001_0000 << 56;
        let black_pawns: u64 = 0b1111_1111_0000_0000 << 40;

        BitBoard {
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_king,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_king,
        }
    }

    pub fn empty() -> Self {
        BitBoard {
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_king: 0,
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_king: 0,
        }
    }

    #[cfg(test)]
    pub fn from_array(arr: [[Option<ChessPiece>; 8]; 8]) -> Self {
        let mut bit_board = BitBoard::empty();
        for (i, row) in arr.into_iter().enumerate() {
            for (j, piece) in row.into_iter().enumerate() {
                if let Some(piece) = piece {
                    let position = Position {
                        x: j as i32,
                        y: i as i32,
                    };
                    bit_board.add_piece_at_position(piece, position);
                }
            }
        }
        bit_board
    }

    pub fn piece_at_bit_board(&self, bit_board: u64) -> Option<ChessPiece> {
        if self.white_pawns & bit_board != 0 {
            return Some(ChessPiece::create_pawn(Color::White));
        } else if self.white_knights & bit_board != 0 {
            return Some(ChessPiece::create_knight(Color::White));
        } else if self.white_bishops & bit_board != 0 {
            return Some(ChessPiece::create_bishop(Color::White));
        } else if self.white_rooks & bit_board != 0 {
            return Some(ChessPiece::create_rook(Color::White));
        } else if self.white_queens & bit_board != 0 {
            return Some(ChessPiece::create_queen(Color::White));
        } else if self.white_king & bit_board != 0 {
            return Some(ChessPiece::create_king(Color::White));
        } else if self.black_pawns & bit_board != 0 {
            return Some(ChessPiece::create_pawn(Color::Black));
        } else if self.black_knights & bit_board != 0 {
            return Some(ChessPiece::create_knight(Color::Black));
        } else if self.black_bishops & bit_board != 0 {
            return Some(ChessPiece::create_bishop(Color::Black));
        } else if self.black_rooks & bit_board != 0 {
            return Some(ChessPiece::create_rook(Color::Black));
        } else if self.black_queens & bit_board != 0 {
            return Some(ChessPiece::create_queen(Color::Black));
        } else if self.black_king & bit_board != 0 {
            return Some(ChessPiece::create_king(Color::Black));
        }
        return None;
    }

    pub fn piece_at_position(&self, position: Position) -> Option<ChessPiece> {
        let position_bit_board = position.to_bit_board();
        self.piece_at_bit_board(position_bit_board)
    }

    pub fn is_position_clean(&self, position: Position) -> bool {
        let position_bit_board = position.to_bit_board();
        let all_pieces = self.full_board();

        all_pieces & position_bit_board == 0
    }

    pub fn full_board(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
            | self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn white_pieces(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn to_array(&self) -> [[Option<ChessPiece>; 8]; 8] {
        let mut array = [[None; 8]; 8];
        for x in 0..8 {
            for y in 0..8 {
                let position = Position { x, y };
                array[x as usize][y as usize] = self.piece_at_position(position);
            }
        }
        array
    }

    pub fn remove_piece_at_position(&mut self, target: Position) {
        let target_bit_board = target.to_bit_board();
        self.white_pawns &= !target_bit_board;
        self.white_knights &= !target_bit_board;
        self.white_bishops &= !target_bit_board;
        self.white_rooks &= !target_bit_board;
        self.white_queens &= !target_bit_board;
        self.white_king &= !target_bit_board;
        self.black_pawns &= !target_bit_board;
        self.black_knights &= !target_bit_board;
        self.black_bishops &= !target_bit_board;
        self.black_rooks &= !target_bit_board;
        self.black_queens &= !target_bit_board;
        self.black_king &= !target_bit_board;
    }

    pub fn add_piece_at_position(&mut self, piece: ChessPiece, target: Position) {
        let target_bit_board = target.to_bit_board();
        match piece.get_color() {
            Color::White => match piece.get_type() {
                Type::Pawn => self.white_pawns |= target_bit_board,
                Type::Knight => self.white_knights |= target_bit_board,
                Type::Bishop => self.white_bishops |= target_bit_board,
                Type::Rook => self.white_rooks |= target_bit_board,
                Type::Queen => self.white_queens |= target_bit_board,
                Type::King => self.white_king |= target_bit_board,
            },
            Color::Black => match piece.get_type() {
                Type::Pawn => self.black_pawns |= target_bit_board,
                Type::Knight => self.black_knights |= target_bit_board,
                Type::Bishop => self.black_bishops |= target_bit_board,
                Type::Rook => self.black_rooks |= target_bit_board,
                Type::Queen => self.black_queens |= target_bit_board,
                Type::King => self.black_king |= target_bit_board,
            },
        }
    }

    pub fn move_piece(&mut self, attacker_piece: ChessPiece, from: Position, to: Position) {
        self.remove_piece_at_position(to);
        self.remove_piece_at_position(from);
        self.add_piece_at_position(attacker_piece, to);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        board::bit_board::BitBoard,
        piece::{self, position::Position, ChessPiece, Color},
    };

    #[test]
    fn test_new() {
        let bit_board = BitBoard::new();
        assert_eq!(bit_board.white_pawns, 0b1111_1111_0000_0000);
        assert_eq!(bit_board.white_knights, 0b01000010);
        assert_eq!(bit_board.white_bishops, 0b00100100);
        assert_eq!(bit_board.white_rooks, 0b10000001);
        assert_eq!(bit_board.white_queens, 0b0000_1000);
        assert_eq!(bit_board.white_king, 0b0001_0000);
        assert_eq!(bit_board.black_pawns, 0b1111_1111_0000_0000 << 40);
        assert_eq!(bit_board.black_knights, 0b01000010 << 56);
        assert_eq!(bit_board.black_bishops, 0b00100100 << 56);
        assert_eq!(bit_board.black_rooks, 0b10000001 << 56);
        assert_eq!(bit_board.black_queens, 0b0000_1000 << 56);
        assert_eq!(bit_board.black_king, 0b0001_0000 << 56);
    }

    #[test]
    fn test_find_pieces() {
        let bit_board = BitBoard::new();
        for ch in ('a'..='h').rev() {
            for i in 1..=2 {
                let poss_str = format!("{}{}", ch, i);
                let position = Position::from_str(&poss_str).unwrap();
                assert!(bit_board.piece_at_position(position).is_some());
            }
            for i in 3..=6 {
                let poss_str = format!("{}{}", ch, i);
                let position = Position::from_str(&poss_str).unwrap();
                assert!(bit_board.piece_at_position(position).is_none());
            }
            for i in 7..=8 {
                let poss_str = format!("{}{}", ch, i);
                let position = Position::from_str(&poss_str).unwrap();
                assert!(bit_board.piece_at_position(position).is_some());
            }
        }
    }

    #[test]
    fn test_a3() {
        let bit_board = BitBoard::new();
        let position = Position::from_str("a3").unwrap();
        let piece = bit_board.piece_at_position(position);
        assert!(piece.is_none());
    }

    #[test]
    fn test_add_piece() {
        let mut bit_board = BitBoard::empty();
        let position = Position::from_str("a2").unwrap();
        let piece = piece::ChessPiece::create_pawn(piece::Color::White);
        bit_board.add_piece_at_position(piece, position);
        let piece = bit_board.piece_at_position(position);
        assert!(piece.is_some());
        assert_eq!(bit_board.white_pawns, 0b0000_0001_0000_0000);
    }

    #[test]
    fn test_from_arr() {
        let white_second_row = [
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
            Some(ChessPiece::create_pawn(Color::White)),
        ];

        let pieces: [[Option<ChessPiece>; 8]; 8] = [
            [None; 8],
            white_second_row,
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
        ];

        let bit_board = BitBoard::from_array(pieces);

        assert_eq!(bit_board.white_pawns, 0b1111_1111_0000_0000);
    }
}
