pub mod bit_vec;
pub mod huffman_tree;
pub mod coding;

#[derive(Debug)]
pub enum Error {
    InvalidData,
}

#[cfg(test)]
mod tests {
    use huffman_tree::*;
    use coding::*;

    #[test]
    fn bytes_to_hufftable_test() {
        let bytes = [0, 0, 0, 1, 1, 2, 2, 3, 255];
        let hufftable = HuffmanTable::from_bytes(&bytes);

        for i in 0..256 {
            let i = i as u8;
            match i {
                0 | 1 | 2 | 3 | 255 => {
                    assert!(hufftable.get(i).is_ok());
                }
                _ => {
                    assert!(!hufftable.get(i).is_ok());
                }
            }
        }
    }

    #[test]
    fn encode_decode_test() {
        let bytes = vec![0, 0, 0, 1, 1, 2, 2, 3, 255];
        let huff_tree = HuffmanTree::from_bytes(&bytes);
        let huff_table = HuffmanTable::from_tree(&huff_tree);
        let encoded = encode(&huff_table, &bytes).unwrap();
        let decoded = decode(&huff_tree, &encoded).unwrap();
        assert!(bytes == decoded);

        let bytes = vec![1, 2, 3, 4, 255, 254, 253, 252];
        let huff_tree = HuffmanTree::from_bytes(&bytes);
        let huff_table = HuffmanTable::from_tree(&huff_tree);
        let encoded = encode(&huff_table, &bytes).unwrap();
        let decoded = decode(&huff_tree, &encoded).unwrap();
        assert!(bytes == decoded);
    }
}
