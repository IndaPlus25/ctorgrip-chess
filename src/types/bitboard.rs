use std::ops::BitOr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard(pub u64);

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitBoard {
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }
}
