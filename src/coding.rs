extern crate std;

use super::huffman_table::HuffmanTable;
use super::bit_vec::BitVec;
use super::Error;

pub fn encode(table: &HuffmanTable, bytes: &[u8]) -> Result<BitVec, Error> {
    let mut encoded = BitVec::new();
    for b in bytes {
        let mut code = table.get(*b)?;
        encoded.append(&mut code);
    }
    Ok(encoded)
}
