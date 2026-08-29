use std::{char, collections::HashMap};

use common::input_reader::read_string;
use priority_queue::PriorityQueue;

#[derive(Default)]
struct HuffmanTree {
    root: Node,
}

#[derive(Default)]
struct Node {
    is_leaf: bool,
    weight: u32,
}

#[derive(Default)]
pub struct Encoder {
    lookup: HashMap<char, u32>,
    tree: HuffmanTree,
}

impl Encoder {
    pub fn get_frequencies(
        self,
        file_path: Option<&str>,
    ) -> Result<PriorityQueue<char, u32>, Box<dyn std::error::Error>> {
        let chars: Vec<char> = read_string(file_path)?.chars().collect();
        let mut freq_map: HashMap<char, u32> = HashMap::new();
        let mut pq: PriorityQueue<char, u32> = PriorityQueue::new();

        for char in chars.iter() {
            let count = freq_map.entry(*char).or_insert(0_u32);
            *count += 1;
        }

        for (key, val) in freq_map.iter() {
            pq.push(*key, *val);
        }

        Ok(pq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_get_frequencies_from_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "aaaa").unwrap();

        let encoder = Encoder::default();
        let output: PriorityQueue<char, u32> = encoder
            .get_frequencies(Some(tmp.path().to_str().unwrap()))
            .unwrap();
        let mut expected: PriorityQueue<char, u32> = PriorityQueue::new();
        expected.push('a', 4);

        assert_eq!(expected, output);
    }
}
