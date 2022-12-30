use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone)]
pub struct BitVector {
    vector: Vec<u64>,
}

impl BitVector {
    pub fn new(bits: usize) -> Self {
        BitVector {
            vector: vec![0; pages(bits)],
        }
    }

    pub fn ones(bits: usize) -> Self {
        let (page, offset) = page_offset(bits);
        let mut bit_band = BitVector {
            vector: vec![u64::MAX; page],
        };

        bit_band.vector.push(u64::MAX >> 64 - offset);

        bit_band
    }

    pub fn set(&mut self, bit: usize) -> bool {
        if bit > self.capacity() {
            self.grow(bit + 1);
        }

        let (page, mask) = page_mask(bit);

        let was_set = self.vector[page] & mask != 0;

        self.vector[page] |= mask;

        !was_set
    }

    pub fn get(&self, bit: usize) -> bool {
        if bit > self.capacity() {
            return false;
        }

        let (page, mask) = page_mask(bit);

        self.vector[page] & mask != 0
    }

    pub fn capacity(&self) -> usize {
        self.vector.len() * 64
    }

    fn grow(&mut self, bits: usize) {
        let pages = pages(bits);

        self.vector.resize(pages, 0);
    }
}

impl Shl<usize> for BitVector {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut copy = self.clone();
        copy <<= rhs;
        copy
    }
}

impl ShlAssign<usize> for BitVector {
    fn shl_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }

        let shift = if rhs > 63 {
            self.shl_assign(rhs - 63);
            63
        } else {
            rhs % 64
        };

        let mut carry = 0;

        for page in 0..pages(self.capacity()) {
            let mut carries_next = self.vector[page] >> (63 - shift);
            carries_next >>= 1;
            self.vector[page] <<= shift;
            self.vector[page] |= carry;
            carry = carries_next;
        }

        if carry > 0 {
            self.vector.push(carry);
        }
    }
}

impl Shr<usize> for BitVector {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut copy = self.clone();
        copy >>= rhs;
        copy
    }
}

impl ShrAssign<usize> for BitVector {
    fn shr_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            return;
        }

        if rhs > 64 {
            self.shr_assign(rhs - 64);
        }

        let shift = rhs % 64;

        let mut carry = 0;

        for page in (0..pages(self.capacity())).rev() {
            let carries_next = self.vector[page] << (64 - shift);
            self.vector[page] >>= rhs;
            self.vector[page] |= carry;
            carry = carries_next;
        }
    }
}

fn page_mask(index: usize) -> (usize, u64) {
    let page = index / 64;
    let mask = 1 << (index % 64);
    (page, mask)
}

fn page_offset(index: usize) -> (usize, usize) {
    (index / 64, index % 64)
}

fn pages(bits: usize) -> usize {
    (bits + 63) / 64
}

#[test]
fn bit_shift() {
    let mut regular = 1;
    let mut bit_vector = BitVector::new(64);
    bit_vector.set(0);
    bit_vector.set(63);

    bit_vector <<= 1;
    regular <<= 1;

    assert_eq!(bit_vector.vector[0], regular);

    bit_vector = BitVector::new(64);

    bit_vector.set(0);

    bit_vector <<= 512;

    assert_eq!(bit_vector.get(512), true);
}
