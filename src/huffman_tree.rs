extern crate std;

use std::collections::{BinaryHeap, HashMap};
use std::{cmp, fmt};

use super::bit_vec::BitVec;
use super::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HuffmanTree {
    Leaf { count: usize, value: u8 },
    Node {
        count: usize,
        child_0: Box<HuffmanTree>,
        child_1: Box<HuffmanTree>,
    },
}

#[derive(Debug, Default)]
pub struct HuffmanTable {
    table: HashMap<u8, BitVec>,
}

impl PartialOrd for HuffmanTree {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for HuffmanTree {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // inverse to make min-heap
        Ord::cmp(&other.get_count(), &self.get_count())
    }
}

impl HuffmanTree {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let counts = HuffmanTree::count_bytes(bytes);
        let mut heap = HuffmanTree::counts_to_heap(&counts);
        HuffmanTree::from_heap(&mut heap)
    }

    pub fn to_table(&self) -> HuffmanTable {
        let mut table = HuffmanTable::default();
        self.to_table_rec(&mut table, BitVec::default());
        table
    }

    fn to_table_rec(&self, table: &mut HuffmanTable, mut prefix: BitVec) {
        match self {
            &HuffmanTree::Leaf { count: _, value: v } => {
                table.insert(v, prefix);
            }
            &HuffmanTree::Node {
                count: _,
                child_0: ref ch0,
                child_1: ref ch1,
            } => {
                let mut prefix_clone = prefix.clone();
                prefix_clone.push(false);
                ch0.to_table_rec(table, prefix_clone);
                prefix.push(true);
                ch1.to_table_rec(table, prefix);
            }
        }
    }

    fn count_bytes(bytes: &[u8]) -> [usize; 256] {
        let mut counts = [0; 256];
        for b in bytes {
            counts[*b as usize] += 1;
        }
        counts
    }

    fn counts_to_heap(counts: &[usize; 256]) -> BinaryHeap<HuffmanTree> {
        counts
            .iter()
            .enumerate()
            .filter(|&(_, c)| *c > 0)
            .map(|(b, c)| {
                     assert!(b < 256);
                     HuffmanTree::Leaf {
                         count: *c,
                         value: b as u8,
                     }
                 })
            .collect()
    }

    fn from_heap(heap: &mut BinaryHeap<HuffmanTree>) -> HuffmanTree {
        assert!(heap.len() > 0);
        if heap.len() == 1 {
            return heap.pop().unwrap();
        }

        while heap.len() > 2 {
            let node_a = heap.pop().unwrap();
            let node_b = heap.pop().unwrap();
            heap.push(HuffmanTree::new_node(node_a, node_b));
        }
        assert!(heap.len() == 2);

        let node_a = heap.pop().unwrap();
        let node_b = heap.pop().unwrap();
        HuffmanTree::new_node(node_a, node_b)
    }

    fn new_node(node_a: HuffmanTree, node_b: HuffmanTree) -> HuffmanTree {
        let count = node_a.get_count() + node_b.get_count();
        let node_a = Box::new(node_a);
        let node_b = Box::new(node_b);
        if *node_a <= *node_b {
            HuffmanTree::Node {
                count: count,
                child_0: node_a,
                child_1: node_b,
            }
        } else {
            HuffmanTree::Node {
                count: count,
                child_0: node_b,
                child_1: node_a,
            }
        }
    }

    fn get_count(&self) -> usize {
        match self {
            &HuffmanTree::Leaf { count: c, .. } => c,
            &HuffmanTree::Node { count: c, .. } => c,
        }
    }
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
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let tree = HuffmanTree::from_bytes(bytes);
        HuffmanTable::from_tree(&tree)
    }

    pub fn from_tree(tree: &HuffmanTree) -> Self {
        tree.to_table()
    }

    pub fn get(&self, byte: u8) -> Result<BitVec, Error> {
        match self.table.get(&byte) {
            Some(code) => Ok(code.clone()),
            None => Err(Error::InvalidData),
        }
    }

    fn insert(&mut self, byte: u8, code: BitVec) {
        self.table.insert(byte, code);
    }
}
