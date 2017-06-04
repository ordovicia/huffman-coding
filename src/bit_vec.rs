//! This module provides `BitVec` struct, vector of bit.

extern crate std;

use std::fmt;

const BYTE_LEN: usize = 8;
const BYTE4_LEN: usize = 32;
type Byte = u8;
type Byte4 = u32;

/// Bit vector.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitVec {
    four_bytes: Vec<Byte4>, // 4-byte boundary alignment
    rem_bits: Byte4,
    bits_len: usize,
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for fb in &self.four_bytes {
            for i in 0..BYTE4_LEN {
                write!(f, "{}", (fb >> (BYTE4_LEN - 1 - i)) & 1).unwrap();
            }
        }
        for i in 0..self.bits_len {
            write!(f, "{}", (self.rem_bits >> (BYTE4_LEN - 1 - i)) & 1).unwrap();
        }
        Ok(())
    }
}

impl BitVec {
    /// Construct `BitVec` from bits (vector of bool).
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    /// let bitvec = huffman_coding::bit_vec::BitVec::from_bits(&vec![true, false]);
    /// ```
    pub fn from_bits(bits: &[bool]) -> Self {
        let len = bits.len();
        let mut four_bytes = Vec::with_capacity(len / BYTE4_LEN);
        let mut i = 0;
        while len - i >= BYTE4_LEN {
            let mut four_byte = 0;
            for j in 0..BYTE4_LEN {
                four_byte += (bits[i + j] as Byte4) << (BYTE4_LEN - 1 - j);
            }
            i += BYTE4_LEN;
            four_bytes.push(four_byte);
        }
        let rem_bits = BitVec::bits_to_byte4(&bits[i..]);

        BitVec {
            four_bytes,
            rem_bits,
            bits_len: len - i,
        }
    }

    /// Construct `BitVec` from bytes (vector of u8).
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    /// let bitvec = huffman_coding::bit_vec::BitVec::from_bytes(&vec![0u8, 255]);
    /// ```
    pub fn from_bytes(bytes: &[Byte]) -> Self {
        let len = bytes.len();
        let mut four_bytes = Vec::with_capacity(len / BYTE_LEN);
        let mut i = 0;
        while len - i >= 4 {
            let mut four_byte = 0;
            for j in 0..4 {
                four_byte += (bytes[i + j] as Byte4) << (BYTE_LEN * (4 - 1 - j));
            }
            i += 4;
            four_bytes.push(four_byte);
        }
        let rem_bits = BitVec::bytes_to_byte4(&bytes[i..]);

        BitVec {
            four_bytes,
            rem_bits,
            bits_len: BYTE_LEN * (len - i),
        }
    }

    /// Appnd another `BitVec` to the back of self.
    /// `other` is left empty.
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    ///
    /// fn main() {
    ///     use huffman_coding::bit_vec::BitVec;
    ///
    ///     let mut bitvec = BitVec::from_bits(&vec![true, false]); // 0b10
    ///     bitvec.append(&mut BitVec::from_bits(&vec![false, true])); // 0b1001
    ///     assert!(bitvec == BitVec::from_bits(&vec![true, false, false, true]));
    ///
    ///     bitvec.append(&mut BitVec::from_bits(&vec![true; 32])); // 0b100111..11
    ///     let mut vec = vec![true, false, false, true];
    ///     vec.append(&mut vec![true; 32]);
    ///     assert!(bitvec == BitVec::from_bits(&vec));
    /// }
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        let mut bit_vec = BitVec::bits_to_vec(self.rem_bits, self.bits_len);
        bit_vec.append(&mut BitVec::four_bytes_to_vec(&other.four_bytes));
        bit_vec.append(&mut BitVec::bits_to_vec(other.rem_bits, other.bits_len));
        let BitVec {
            mut four_bytes,
            rem_bits,
            bits_len,
        } = BitVec::from_bits(&bit_vec);

        self.four_bytes.append(&mut four_bytes);
        self.rem_bits = rem_bits;
        self.bits_len = bits_len;

        other.rem_bits = 0;
        other.bits_len = 0;
    }

    /// Append a bit to the back of self.
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    ///
    /// fn main() {
    ///     use huffman_coding::bit_vec::BitVec;
    ///
    ///     let mut bitvec = BitVec::from_bits(&vec![true, false]); // 0b10
    ///     bitvec.push(true);
    ///     assert!(bitvec == BitVec::from_bits(&vec![true, false, true])); // 0b101
    ///
    ///     bitvec.push(false);
    ///     assert!(bitvec == BitVec::from_bits(&vec![true, false, true, false])); // 0b1010
    /// }
    /// ```
    pub fn push(&mut self, bit: bool) {
        if self.bits_len == BYTE4_LEN - 1 {
            self.four_bytes.push(self.rem_bits | (bit as Byte4));
            self.rem_bits = 0;
            self.bits_len = 0;
        } else {
            assert!(self.bits_len < BYTE4_LEN - 1);
            self.rem_bits |= (bit as Byte4) << (BYTE4_LEN - 1 - self.bits_len);
            self.bits_len += 1;
        }
    }

    /// Returns the number of bits.
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    ///
    /// fn main() {
    ///     use huffman_coding::bit_vec::BitVec;
    ///
    ///     let mut bitvec = BitVec::from_bits(&vec![true, false]); // 0b10
    ///     assert!(bitvec.len() == 2);
    ///
    ///     bitvec.append(&mut BitVec::from_bits(&vec![true, false, true, false])); // 0b101010
    ///     assert!(bitvec.len() == 6);
    ///
    ///     let vec = vec![true; 32];
    ///     bitvec.append(&mut BitVec::from_bits(&vec)); // 0b10101011..11
    ///     assert!(bitvec.len() == 38);
    /// }
    /// ```
    pub fn len(&self) -> usize {
        BYTE4_LEN * self.four_bytes.len() + self.bits_len
    }

    /// Align to the 4-byte boundary by padding 0.
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    ///
    /// fn main() {
    ///     use huffman_coding::bit_vec::BitVec;
    ///
    ///     let mut vec = vec![true; 32];
    ///     let mut bitvec = BitVec::from_bits(&vec);
    ///     bitvec.align();
    ///     assert!(bitvec == BitVec::from_bits(&vec));
    ///
    ///     let mut vec2 = vec![true, false];
    ///     bitvec.append(&mut BitVec::from_bits(&vec2));
    ///     bitvec.align();
    ///     vec2.append(&mut vec![false; 30]);
    ///     vec.append(&mut vec2);
    ///     assert!(bitvec == BitVec::from_bits(&vec));
    /// }
    /// ```
    pub fn align(&mut self) {
        if self.bits_len > 0 {
            self.four_bytes.push(self.rem_bits);
            self.rem_bits = 0;
            self.bits_len = 0;
        }
    }

    /// Returns aligned copy.
    ///
    /// # Examples
    /// ```
    /// extern crate huffman_coding;
    ///
    /// fn main() {
    ///     use huffman_coding::bit_vec::BitVec;
    ///
    ///     let mut bitvec = BitVec::from_bits(&vec![true; 32]);
    ///     assert!(bitvec.aligned_four_bytes() == vec![std::u32::MAX]);
    ///
    ///     bitvec.append(&mut BitVec::from_bits(&vec![true, false]));
    ///     assert!(bitvec.aligned_four_bytes() == vec![std::u32::MAX, 1u32 << 31]);
    /// }
    /// ```
    pub fn aligned_four_bytes(&self) -> Vec<Byte4> {
        let mut four_bytes = self.four_bytes.clone();
        if self.bits_len > 0 {
            four_bytes.push(self.rem_bits);
        }
        four_bytes
    }

    fn bits_to_vec(bits: Byte4, len: usize) -> Vec<bool> {
        let mut vec = vec![false; len];
        for i in 0..len {
            vec[i] = (bits >> (BYTE4_LEN - 1 - i)) & 1 == 1;
        }
        vec
    }

    fn four_bytes_to_vec(four_bytes: &[Byte4]) -> Vec<bool> {
        let len = BYTE4_LEN * four_bytes.len();
        let mut vec = vec![false; len];
        for i in 0..len {
            vec[i] = (four_bytes[i / BYTE4_LEN] >> (BYTE4_LEN - 1 - i)) & 1 == 1;
        }
        vec
    }

    fn bits_to_byte4(bits: &[bool]) -> Byte4 {
        assert!(bits.len() < BYTE4_LEN);
        let mut bits_byte4 = 0;
        for i in 0..bits.len() {
            bits_byte4 += (bits[i] as Byte4) << (BYTE4_LEN - 1 - i);
        }
        bits_byte4
    }

    fn bytes_to_byte4(bytes: &[Byte]) -> Byte4 {
        assert!(bytes.len() < 4);
        let mut bits_byte4 = 0;
        for i in 0..bytes.len() {
            bits_byte4 += (bytes[i] as Byte4) << (4 - 1 - i);
        }
        bits_byte4
    }
}

/// Iterate all bits in `BitVec`.
///
/// # Examples
/// ```
/// extern crate huffman_coding;
///
/// fn main() {
///     use huffman_coding::bit_vec::BitVec;
///
///     let bitvec = BitVec::from_bits(&vec![true, false]); // 0b10
///     let mut iter = bitvec.into_iter();
///     assert!(iter.next() == Some(true));
///     assert!(iter.next() == Some(false));
///     assert!(iter.next() == None);
///
///     let mut vec = vec![true; 20];
///     vec.append(&mut vec![false; 20]); // 0b11..1100..00
///     let bitvec = BitVec::from_bits(&vec);
///     let mut iter = bitvec.into_iter();
///     for _ in 0..20 {
///         assert!(iter.next() == Some(true));
///     }
///     for _ in 0..20 {
///         assert!(iter.next() == Some(false));
///     }
///     assert!(iter.next() == None);
/// }
/// ```
pub struct BitVecIterator<'a> {
    bitvec: &'a BitVec,
    four_bytes_idx: usize,
    bits_idx: usize,
}

impl<'a> IntoIterator for &'a BitVec {
    type Item = bool;
    type IntoIter = BitVecIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitVecIterator {
            bitvec: self,
            four_bytes_idx: 0,
            bits_idx: 0,
        }
    }
}

impl<'a> Iterator for BitVecIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.bits_idx == BYTE4_LEN {
            self.four_bytes_idx += 1;
            self.bits_idx = 0;
        }

        // println!("{}/{} {}/{}",
        //          self.four_bytes_idx,
        //          self.bitvec.four_bytes.len(),
        //          self.bits_idx,
        //          self.bitvec.bits_len);

        if self.four_bytes_idx >= self.bitvec.four_bytes.len() &&
           self.bits_idx >= self.bitvec.bits_len {
            None
        } else {
            let ret = ((if self.four_bytes_idx == self.bitvec.four_bytes.len() {
                            self.bitvec.rem_bits
                        } else {
                            self.bitvec.four_bytes[self.four_bytes_idx]
                        }) >> (BYTE4_LEN - 1 - self.bits_idx)) & 1 == 1;

            self.bits_idx += 1;
            Some(ret)
        }
    }
}
