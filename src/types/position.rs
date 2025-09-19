use super::bitboard::Bitboard;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

pub enum Color {
    White,
    Black,
}

pub struct Position {
    pub white_pawns: Bitboard,
    pub white_knights: Bitboard,
    pub white_bishops: Bitboard,
    pub white_queens: Bitboard,
    pub white_king: Bitboard,
    pub black_pawns: Bitboard,
    pub black_knights: Bitboard,
    pub black_bishops: Bitboard,
    pub black_queens: Bitboard,
    pub black_king: Bitboard,
    pub turn: Color,
}

impl Position {
    pub fn default() -> Self {
        Self {
            white_pawns: BitBoard::default_pawns(),
            white_knights: BitBoard::default_knights(),
            white_bishops: BitBoard::default_bishops(),
            white_queens: BitBoard::default_queens(),
            white_king: BitBoard::default_kings(),
            black_pawns: BitBoard::default_pawns(),
            black_knights: BitBoard::default_knights(),
            black_bishops: BitBoard::default_bishops(),
            black_queens: BitBoard::default_queens(),
            black_king: BitBoard::default_king(),
            turn: Color::White
        }
    }
}