pub mod bit_vec;
pub mod huffman_tree;
pub mod huffman_table;
pub mod coding;

#[derive(Debug)]
pub enum Error {
    InvalidData,
}

#[cfg(test)]
mod tests {
    // use bit_vec::*;
    use huffman_tree::*;
    // use huffman_table::*;
    use coding::*;

    #[test]
    fn from_bytes_to_table_test() {
        let bytes = [0, 0, 0, 1, 1, 2, 2, 3, 255];
        let tree = HuffmanTree::from_bytes(&bytes);
        let table = tree.to_table();

        // println!("tree: {:?}", tree);
        // println!("table: {}", table);

        for i in 0..256 {
            let i = i as u8;
            match i {
                0 | 1 | 2 | 3 | 255 => {
                    assert!(table.get(i).is_ok());
                }
                _ => {
                    assert!(!table.get(i).is_ok());
                }
            }
        }
    }

    #[test]
    fn encode_test() {
        let bytes = [0, 0, 0, 1, 1, 2, 2, 3, 255];
        let table = HuffmanTree::from_bytes(&bytes).to_table();
        let encoded = encode(&table, &bytes).unwrap();

        println!("bytes: {:?}", bytes);
        println!("table: {}", table);
        println!("encoded: {}", encoded);
    }
}
