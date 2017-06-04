extern crate std;

use super::huffman_tree::{HuffmanTree, HuffmanTable};
use super::bit_vec::BitVec;
use super::Error;

pub fn encode(table: &HuffmanTable, bytes: &[u8]) -> Result<BitVec, Error> {
    let mut encoded = BitVec::default();
    for b in bytes {
        let mut code = table.get(*b)?;
        encoded.append(&mut code);
    }
    Ok(encoded)
}

pub fn decode(tree: &HuffmanTree, bits: &BitVec) -> Result<Vec<u8>, Error> {
    let mut decoded = vec![];
    let mut state = tree;

    for bit in bits.into_iter() {
        match state {
            &HuffmanTree::Node {
                count: _,
                child_0: ref ch0,
                child_1: ref ch1,
            } => {
                if bit {
                    state = ch1;
                } else {
                    state = ch0;
                }

                if let &HuffmanTree::Leaf { count: _, value: v } = state {
                    decoded.push(v);
                    state = &tree;
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    match state {
        &HuffmanTree::Node { .. } => Ok(decoded),
        _ => Err(Error::InvalidData),
    }
}
