pub struct BitBoard(u64);

impl Bitboard {
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub fn default_pawns() -> Self {
        Self(0b1111111100000000)
    }

    pub fn default_knights() -> Self {
        Self(0b1000010)
    }

    pub fn default_bishops() -> Self {
        Self(0b100100)
    }

    pub fn default_queens() -> Self {
        Self(0b10000)
    }

    pub fn default_king() -> Self {
        Self(0b1000)
    }
}