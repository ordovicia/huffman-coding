extern crate std;

use std::collections::HashMap;
use std::fmt;

use super::bit_vec::BitVec;
use super::Error;

#[derive(Default)]
pub struct HuffmanTable {
    table: HashMap<u8, BitVec>,
}

impl fmt::Display for HuffmanTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (b, e) in &self.table {
            write!(f, "{}: {}, ", *b, *e).unwrap();
        }
        Ok(())
    }
}

impl HuffmanTable {
    pub fn get(&self, byte: u8) -> Result<BitVec, Error> {
        match self.table.get(&byte) {
            Some(code) => Ok(code.clone()),
            None => Err(Error::InvalidData),
        }
    }

    pub fn insert(&mut self, byte: u8, code: BitVec) {
        self.table.insert(byte, code);
    }
}
