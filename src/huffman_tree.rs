extern crate std;

use std::collections::BinaryHeap;
use std::cmp;

use super::bit_vec::BitVec;
use super::huffman_table::HuffmanTable;

#[derive(Debug, PartialEq, Eq)]
pub enum HuffmanTree {
    Leaf { count: usize, value: u8 },
    Node {
        count: usize,
        child_0: Box<HuffmanTree>,
        child_1: Box<HuffmanTree>,
    },
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
        self.to_table_rec(&mut table, BitVec::new());
        table
    }

    fn to_table_rec(&self, table: &mut HuffmanTable, prefix: BitVec) {
        match self {
            &HuffmanTree::Leaf { count: _, value: v } => {
                table.insert(v, prefix);
            }
            &HuffmanTree::Node {
                count: _,
                child_0: ref ch0,
                child_1: ref ch1,
            } => {
                ch0.to_table_rec(table, prefix.clone_push(false));
                ch1.to_table_rec(table, prefix.clone_push(true));
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
