extern crate std;

use std::fmt;

#[derive(Debug, Clone)]
pub struct BitVec {
    bytes: Vec<u8>,
    bits: Vec<bool>,
}

pub struct BitVecIterator<'a> {
    bitvec: &'a BitVec,
    byte_idx: usize,
    bit_idx: usize,
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for byte in &self.bytes {
            for i in 0..8 {
                write!(f, "{}", (byte >> (7 - i)) & 1u8).unwrap();
            }
        }
        for bit in &self.bits {
            write!(f, "{}", if *bit { 1 } else { 0 }).unwrap();
        }
        Ok(())
    }
}

impl BitVec {
    pub fn new() -> Self {
        BitVec {
            bytes: Vec::new(),
            bits: Vec::with_capacity(8),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.bits.append(&mut BitVec::bytes_to_bits(&other.bytes));
        self.bits.append(&mut other.bits);
        let (mut new_bytes, new_bits) = BitVec::bits_to_bytes(&self.bits);
        self.bytes.append(&mut new_bytes);
        self.bits = new_bits;
    }

    pub fn push(&mut self, bit: bool) {
        assert!(self.bits.len() < 8);
        match self.bits.len() {
            7 => {
                self.bits.push(bit);
                let new_byte = BitVec::bits8_to_bytes(&self.bits);
                self.bytes.push(new_byte);
                self.bits.clear();
            }
            l if l < 8 => {
                self.bits.push(bit);
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub fn clone_push(&self, bit: bool) -> Self {
        let mut cloned = self.clone();

        match cloned.bits.len() {
            7 => {
                cloned.bits.push(bit);
                let new_byte = BitVec::bits8_to_bytes(&cloned.bits);
                cloned.bytes.push(new_byte);
                cloned.bits.clear();
            }
            l if l < 8 => {
                cloned.bits.push(bit);
            }
            _ => {
                unreachable!();
            }
        }

        cloned
    }

    fn bytes_to_bits(bytes: &Vec<u8>) -> Vec<bool> {
        let mut bits = Vec::with_capacity(bytes.len() * 8);
        for byte in bytes {
            for i in 0..8 {
                bits.push(((byte >> (7 - i)) & 1u8) == 1u8);
            }
        }
        bits
    }

    fn bits_to_bytes(bits: &Vec<bool>) -> (Vec<u8>, Vec<bool>) {
        let mut byte = 0u8;
        let mut bytes = vec![];
        let mut i = 0;
        while bits.len() - i >= 8 {
            for j in 0..8 {
                byte += (bits[i + j] as u8) << (7 - j);
            }
            i += 8;
            bytes.push(byte);
            byte = 0;
        }
        (bytes, bits[i..].to_vec())
    }

    fn bits8_to_bytes(bits: &Vec<bool>) -> u8 {
        assert!(bits.len() == 8);
        let mut byte = 0u8;
        for i in 0..8 {
            byte += (bits[i] as u8) << (7 - i);
        }
        byte
    }
}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = BitVecIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitVecIterator {
            bitvec: self,
            byte_idx: 0,
            bit_idx: 0,
        }
    }
}

impl<'a> Iterator for BitVecIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.bit_idx == 8 {
            self.bit_idx = 0;
            self.byte_idx += 1;
        }

        if self.byte_idx >= self.bitvec.bytes.len() && self.bit_idx >= self.bitvec.bits.len() {
            None
        } else {
            let ret = if self.byte_idx == self.bitvec.bytes.len() {
                self.bitvec.bits[self.bit_idx]
            } else {
                (self.bitvec.bytes[self.byte_idx] >> (7 - self.bit_idx)) & 1u8 == 1u8
            };

            self.bit_idx += 1;
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitVec;

    #[test]
    fn append_test() {
        let mut bitvec = BitVec {
            bytes: vec![0, 255],
            bits: vec![],
        };
        bitvec.append(&mut BitVec {
                               bytes: vec![255, 0],
                               bits: vec![],
                           });
        assert!(bitvec.bytes == vec![0, 255, 255, 0]);
        assert!(bitvec.bits == vec![]);

        bitvec = BitVec {
            bytes: vec![],
            bits: vec![true, false, true, false],
        };
        bitvec.append(&mut BitVec {
                               bytes: vec![],
                               bits: vec![false, true, false, true],
                           });
        assert!(bitvec.bytes == vec![165]);
        assert!(bitvec.bits == vec![]);

        bitvec = BitVec {
            bytes: vec![255],
            bits: vec![true, false, true, false],
        };
        bitvec.append(&mut BitVec {
                               bytes: vec![16], // 0001'0000
                               bits: vec![false, true, false, true, false],
                           });
        assert!(bitvec.bytes == vec![255, 161, 5]); // 255, 1010'0001, 0000'0101
        assert!(bitvec.bits == vec![false]);
    }

    #[test]
    fn push_test() {
        let mut bitvec = BitVec {
            bytes: vec![],
            bits: vec![],
        };

        bitvec.push(true);
        assert!(bitvec.bits == vec![true]);
        bitvec.push(true);
        assert!(bitvec.bits == vec![true, true]);
        bitvec.push(false);
        bitvec.push(false);
        assert!(bitvec.bits == vec![true, true, false, false]);
        bitvec.push(true);
        bitvec.push(false);
        bitvec.push(false);
        bitvec.push(true); // 1100'1001
        assert!(bitvec.bytes == vec![201]);
        assert!(bitvec.bits == vec![]);
        bitvec.push(true);
        assert!(bitvec.bytes == vec![201]);
        assert!(bitvec.bits == vec![true]);
    }

    #[test]
    fn bits8_to_bytes_test() {
        let mut bits = vec![false; 8];
        assert!(BitVec::bits8_to_bytes(&bits) == 0u8);
        bits[0] = true; // 1000'0000
        assert!(BitVec::bits8_to_bytes(&bits) == 128u8);
        bits[1] = true; // 1100'0000
        assert!(BitVec::bits8_to_bytes(&bits) == 192u8);
        bits[2] = true; // 1110'0000
        assert!(BitVec::bits8_to_bytes(&bits) == 224u8);
        bits[1] = false; // 1010'0000
        assert!(BitVec::bits8_to_bytes(&bits) == 160u8);
        bits[5] = true; // 1010'0100
        assert!(BitVec::bits8_to_bytes(&bits) == 164u8);
        bits[7] = true; // 1010'0101
        assert!(BitVec::bits8_to_bytes(&bits) == 165u8);
    }

    #[test]
    fn bits_to_bytes_test() {
        use super::BitVec;

        let bits = vec![true];
        assert!(BitVec::bits_to_bytes(&bits) == (vec![], vec![true]));

        let mut bits = vec![true, false, true, false];
        bits.append(&mut vec![true, false, true, false]);
        // 1010'1010
        assert!(BitVec::bits_to_bytes(&bits) == (vec![170u8], vec![]));

        bits.append(&mut vec![true, false]);
        // 1010'1010 + 10
        assert!(BitVec::bits_to_bytes(&bits) == (vec![170u8], vec![true, false]));
        bits.append(&mut vec![true, false, true, true, false, false]);
        // 1010'1010'1010'1100
        assert!(BitVec::bits_to_bytes(&bits) == (vec![170u8, 172u8], vec![]));

        bits.append(&mut vec![true, false]);
        // 1010'1010'1010'1100 + 10
        assert!(BitVec::bits_to_bytes(&bits) == (vec![170u8, 172u8], vec![true, false]));
    }

    #[test]
    fn iterator_test() {
        let bitvec = BitVec {
            bytes: vec![170], /* 10101010 */
            bits: vec![true, false],
        };

        let mut iter = bitvec.into_iter();
        assert!(iter.next() == Some(true));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(true));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(true));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(true));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(true));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == None);
    }
}
