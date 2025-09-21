use super::bitboard::BitBoard;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Pieces: Bitboard representation of all occupied squares by each piece type
/// [Pawns, Knights, Bishops, Rooks, Queens, Kings]
///
/// Colors: Bitboard representation of all occupied squares by each color
/// [White, Black]
///
/// Side to Move: The color to move in the current position
/// Color::{White/Black}
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub pieces: [BitBoard; 6],
    pub colors: [BitBoard; 2],
    pub side_to_move: Color,
}

impl Position {
    /// A default starting position
    pub fn default() -> Self {
        Self {
            pieces: [
                // Hexes used for brevity, refer to:
                // https://tearth.dev/bitboard-viewer/
                // for visualising the bits
                BitBoard(0xFF00000000FF00),
                BitBoard(0x4200000000000042),
                BitBoard(0x2400000000000024),
                BitBoard(0x8100000000000081),
                BitBoard(0x800000000000008),
                BitBoard(0x1000000000000010),
            ],
            colors: [BitBoard(0xFFFF), BitBoard(0xFFFF000000000000)],
            side_to_move: Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_default() {
        let default_pos = Position::default();
        let both_colors = BitBoard(0xffff00000000ffff);
        assert_eq!(default_pos.colors[0] | default_pos.colors[1], both_colors);
        assert_eq!(default_pos.side_to_move, Color::White);
    }
}
