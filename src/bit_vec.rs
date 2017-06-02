extern crate std;

use std::fmt;

#[derive(Debug, Clone)]
pub struct BitVec {
    vec: Vec<bool>,
}

impl fmt::Display for BitVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for b in &self.vec {
            if *b {
                write!(f, "1").unwrap();
            } else {
                write!(f, "0").unwrap();
            }
        }
        Ok(())
    }
}

impl BitVec {
    pub fn new() -> Self {
        BitVec { vec: Vec::new() }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.vec.append(&mut other.vec);
    }

    pub fn clone_push(&self, v: bool) -> Self {
        let mut cloned = self.vec.clone();
        cloned.push(v);
        BitVec { vec: cloned }
    }
}
