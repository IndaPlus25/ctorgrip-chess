use std::ops::{BitAnd, BitOr, ShlAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BitBoard(pub u64);

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitBoard {
    pub fn empty() -> Self {
        Self(0)
    }
}
