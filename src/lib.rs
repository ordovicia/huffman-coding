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
    fn encode_decode_test() {
        let bytes = vec![0, 0, 0, 1, 1, 2, 2, 3, 255];
        let huff_tree = HuffmanTree::from_bytes(&bytes);
        let huff_table = HuffmanTable::from_tree(&huff_tree);
        let encoded = encode(&huff_table, &bytes).unwrap();
        let decoded = decode(&huff_tree, &encoded).unwrap();
        assert!(bytes == decoded);

        let mut bytes = vec![1; 1];
        bytes.append(&mut vec![2; 3]);
        bytes.append(&mut vec![4; 6]);
        bytes.append(&mut vec![5; 1]);
        bytes.append(&mut vec![7; 3]);
        bytes.append(&mut vec![10; 5]);
        bytes.append(&mut vec![19; 2]);
        bytes.append(&mut vec![149; 4]);
        bytes.append(&mut vec![255; 5]);
        bytes.append(&mut vec![10; 2]);
        bytes.append(&mut vec![1; 5]);
        let huff_tree = HuffmanTree::from_bytes(&bytes);
        let huff_table = HuffmanTable::from_tree(&huff_tree);
        let encoded = encode(&huff_table, &bytes).unwrap();
        let decoded = decode(&huff_tree, &encoded).unwrap();
        assert!(bytes == decoded);
    }
}
