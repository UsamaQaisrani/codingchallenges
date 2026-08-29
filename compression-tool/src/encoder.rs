use std::{char, collections::HashMap};

use common::input_reader::read_string;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[derive(Debug, Default, Hash, PartialEq, Eq)]
struct HuffmanNode {
    pub character: Option<char>,
    pub frequency: u32,
    left_child: Option<Box<HuffmanNode>>,
    right_child: Option<Box<HuffmanNode>>,
}

#[derive(Default, Clone)]
pub struct Encoder;

impl Encoder {
    fn get_frequencies(
        self,
        file_path: Option<&str>,
    ) -> Result<PriorityQueue<HuffmanNode, Reverse<u32>>, Box<dyn std::error::Error>> {
        let chars: Vec<char> = read_string(file_path)?.chars().collect();
        let mut freq_map: HashMap<char, u32> = HashMap::new();
        let mut pq: PriorityQueue<HuffmanNode, Reverse<u32>> = PriorityQueue::new();

        for char in chars.iter() {
            let count = freq_map.entry(*char).or_insert(0_u32);
            *count += 1;
        }

        for (key, val) in freq_map.iter() {
            pq.push(
                HuffmanNode {
                    character: Some(*key),
                    frequency: *val,
                    left_child: None,
                    right_child: None,
                },
                Reverse(*val),
            );
        }

        Ok(pq)
    }

    fn build_huffman_tree(
        &self,
        pq: &mut PriorityQueue<HuffmanNode, Reverse<u32>>,
    ) -> Result<HuffmanNode, Box<dyn std::error::Error>> {
        if pq.is_empty() {
            return Ok(HuffmanNode::default());
        }
        while pq.len() >= 2 {
            let Some((left, _)) = pq.pop() else {
                break;
            };

            let Some((right, _)) = pq.pop() else {
                break;
            };

            let parent_freq = left.frequency + right.frequency;
            let parent: HuffmanNode = HuffmanNode {
                character: None,
                frequency: parent_freq,
                left_child: Some(Box::new(left)),
                right_child: Some(Box::new(right)),
            };

            pq.push(parent, Reverse(parent_freq));
        }

        let Some((root, _)) = pq.pop() else {
            return Err("Empty queue, cannot build HuffmanTree".into());
        };

        Ok(root)
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

        let encoder = Encoder {};
        let output: PriorityQueue<HuffmanNode, Reverse<u32>> = encoder
            .get_frequencies(Some(tmp.path().to_str().unwrap()))
            .unwrap();
        let mut expected: PriorityQueue<HuffmanNode, Reverse<u32>> = PriorityQueue::new();
        expected.push(
            HuffmanNode {
                character: Some('a'),
                frequency: 4,
                left_child: None,
                right_child: None,
            },
            Reverse(4),
        );

        assert_eq!(expected, output);
    }

    #[test]
    fn test_build_huffman_tree() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "aaaabb").unwrap();

        let encoder = Encoder {};
        let mut pq: PriorityQueue<HuffmanNode, Reverse<u32>> = encoder
            .clone()
            .get_frequencies(Some(tmp.path().to_str().unwrap()))
            .unwrap();
        let output = encoder.build_huffman_tree(&mut pq).unwrap();
        let left_child = HuffmanNode {
            character: Some('b'),
            frequency: 2,
            left_child: None,
            right_child: None,
        };

        let right_child = HuffmanNode {
            character: Some('a'),
            frequency: 4,
            left_child: None,
            right_child: None,
        };

        let expected = HuffmanNode {
            character: None,
            frequency: right_child.frequency + left_child.frequency,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        };

        assert_eq!(output, expected);
    }
}
