use super::bitboard::BitBoard;

pub const RANK_4: u64 = 0xff000000;
pub const FILE_H: u64 = 0x8080808080808080;

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
                // Hexes used for brevity, refer to: https://tearth.dev/bitboard-viewer/
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

    pub fn occupied(&self) -> BitBoard {
        self.colors[0] | self.colors[1]
    }

    pub fn pawn_moves(&self) -> BitBoard {
        let mut moves = BitBoard::empty();
        let occupied = self.occupied().0;

        match self.side_to_move {
            Color::White => {
                let pawns = self.pieces[0] & self.colors[0];

                // "Quiets" (non-attacks) - UP 1 (UP 2 : MASKED ON RANK 4)
                let quiets = ((pawns.0 << 8) | ((pawns.0 << 16) & RANK_4)) & !occupied;

                // Attacks
                // 1 UP 1 LEFT : MASKED AWAY H FILE
                (our_pawns.0 << 7) & !FILE_H;
            }
            Color::Black => {
                todo!()
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let default_pos = Position::default();
        let both_colors = BitBoard(0xffff00000000ffff);
        assert_eq!(default_pos.colors[0] | default_pos.colors[1], both_colors);
        assert_eq!(default_pos.side_to_move, Color::White);
    }

    #[test]
    fn occupied() {
        let pos1 = Position::default();
        assert_eq!(pos1.occupied().0, 0xffff00000000ffff)
    }
}
