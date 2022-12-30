use std::ops::{ShlAssign, ShrAssign};

use crate::bit_vector::BitVector;

#[derive(Clone)]
pub struct BitBand {
    pub band: Vec<BitVector>,
}

impl ShlAssign<usize> for BitBand {
    fn shl_assign(&mut self, rhs: usize) {
        for vector in &mut self.band {
            *vector <<= rhs;
        }
    }
}

impl ShrAssign<usize> for BitBand {
    fn shr_assign(&mut self, rhs: usize) {
        for vector in &mut self.band {
            *vector >>= rhs;
        }
    }
}
