use std::{char, collections::HashMap, io::Bytes};

use common::input_reader::read_string;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::io::Write;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct HuffmanNode {
    pub character: Option<char>,
    pub frequency: u32,
    left_child: Option<Box<HuffmanNode>>,
    right_child: Option<Box<HuffmanNode>>,
}

#[derive(Default, Clone)]
pub struct Encoder<'a> {
    input_file_path: Option<&'a str>,
    output_file_path: String,
}

impl<'a> Encoder<'a> {
    pub fn new(input_file_path: Option<&'a str>, output_file_path: String) -> Self {
        Self {
            input_file_path,
            output_file_path,
        }
    }

    pub fn encode(&self) -> Result<(), Box<dyn std::error::Error>> {
        let chars: Vec<char> = read_string(self.input_file_path)?.chars().collect();
        let freqs = self.get_frequencies(&chars);
        let mut pq = self.create_pq(&freqs);
        let tree = self.build_huffman_tree(&mut pq)?;
        let lookup_table = self.generate_prefix_code_table(tree);
        self.write_encoded_file(&chars, &self.output_file_path, freqs, lookup_table)?;

        Ok(())
    }

    fn get_frequencies(&self, chars: &[char]) -> HashMap<char, u32> {
        let mut freq_map: HashMap<char, u32> = HashMap::new();

        for char in chars.iter() {
            let count = freq_map.entry(*char).or_insert(0_u32);
            *count += 1;
        }

        freq_map
    }

    fn create_pq(&self, freqs: &HashMap<char, u32>) -> PriorityQueue<HuffmanNode, Reverse<u32>> {
        let mut pq: PriorityQueue<HuffmanNode, Reverse<u32>> = PriorityQueue::new();
        for (key, val) in freqs.iter() {
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

        pq
    }

    fn build_huffman_tree(
        &self,
        pq: &mut PriorityQueue<HuffmanNode, Reverse<u32>>,
    ) -> Result<HuffmanNode, Box<dyn std::error::Error>> {
        if pq.is_empty() {
            return Ok(HuffmanNode::default());
        }

        if pq.len() == 1 {
            let Some((child, _)) = pq.pop() else {
                return Err("Empty queue, cannot build HuffmanTree".into());
            };

            let parent = HuffmanNode {
                character: None,
                frequency: child.frequency,
                left_child: Some(Box::new(child)),
                right_child: None,
            };

            return Ok(parent);
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

    fn generate_prefix_code_table(&self, node: HuffmanNode) -> HashMap<char, (u32, u32)> {
        let mut table = HashMap::new();
        self.traverse(&node, 0, 0, &mut table);
        table
    }

    fn traverse(
        &self,
        node: &HuffmanNode,
        code: u32,
        length: u32,
        table: &mut HashMap<char, (u32, u32)>,
    ) {
        if let Some(character) = node.character {
            table.insert(character, (code, length));
            return;
        };

        if let Some(left) = &node.left_child {
            self.traverse(left, code << 1, length + 1, table);
        }

        if let Some(right) = &node.right_child {
            self.traverse(right, (code << 1) | 1, length + 1, table);
        }
    }

    fn write_encoded_file(
        &self,
        chars: &Vec<char>,
        output_file_path: &str,
        freq_map: HashMap<char, u32>,
        lookup_table: HashMap<char, (u32, u32)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        let header = self.create_header(freq_map);
        let encoded_text = self.encode_text(chars, lookup_table);

        bytes.extend_from_slice(&header);
        bytes.extend_from_slice(&encoded_text);

        let mut file = std::fs::File::create(output_file_path)?;
        Write::write_all(&mut file, &bytes)?;

        Ok(())
    }

    fn create_header(&self, table: HashMap<char, u32>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let map_length: u32 = table.len() as u32;

        // Add characters count
        bytes.extend_from_slice(&map_length.to_be_bytes());

        // Add characters and their frequencies
        for (key, val) in table.iter() {
            let c: u32 = *key as u32;
            bytes.extend_from_slice(&c.to_be_bytes());
            bytes.extend_from_slice(&val.to_be_bytes());
        }

        bytes
    }

    fn encode_text(&self, chars: &Vec<char>, table: HashMap<char, (u32, u32)>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut current_byte: u8 = 0;
        let mut bits_in_bytes: u8 = 0;

        for character in chars {
            let &(code, length) = table
                .get(character)
                .expect("character should be in the Huffman Table");

            for i in (0..length).rev() {
                let bit = ((code >> i) & 1) as u8;
                current_byte = (current_byte << 1) | bit;
                bits_in_bytes += 1;

                if bits_in_bytes == 8 {
                    bytes.push(current_byte);
                    current_byte = 0;
                    bits_in_bytes = 0;
                }
            }
        }

        if bits_in_bytes > 0 {
            current_byte <<= 8 - bits_in_bytes;
            bytes.push(current_byte);
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn test_get_frequencies() {
        let encoder = Encoder::default();
        let output = encoder.get_frequencies(&chars("aaaa"));

        let mut expected = HashMap::new();
        expected.insert('a', 4);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_create_pq() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaa"));
        let pq = encoder.create_pq(&freqs);

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

        assert_eq!(pq, expected);
    }

    #[test]
    fn test_build_huffman_tree() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaabb"));
        let mut pq = encoder.create_pq(&freqs);
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

    #[test]
    fn test_generate_prefix_code_table() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaabbc"));
        let mut pq = encoder.create_pq(&freqs);
        let tree = encoder.build_huffman_tree(&mut pq).unwrap();
        let output = encoder.generate_prefix_code_table(tree);

        assert_eq!(output.get(&'a'), Some(&(1, 1)));
        assert_eq!(output.get(&'b'), Some(&(1, 2)));
        assert_eq!(output.get(&'c'), Some(&(0, 2)));
    }

    #[test]
    fn test_generate_prefix_code_table_single_symbol() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("a"));
        let mut pq = encoder.create_pq(&freqs);
        let tree = encoder.build_huffman_tree(&mut pq).unwrap();
        let output = encoder.generate_prefix_code_table(tree);

        assert_eq!(output.get(&'a'), Some(&(0, 1)));
    }

    #[test]
    fn test_create_header() {
        let encoder = Encoder::default();
        let mut freqs = HashMap::new();
        freqs.insert('a', 4_u32);

        let output = encoder.create_header(freqs);
        let mut expected: Vec<u8> = Vec::new();
        expected.extend_from_slice(&1_u32.to_be_bytes());
        expected.extend_from_slice(&('a' as u32).to_be_bytes());
        expected.extend_from_slice(&4_u32.to_be_bytes());

        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_text() {
        let encoder = Encoder::default();
        let mut table = HashMap::new();
        table.insert('a', (0_u32, 1_u32));
        table.insert('b', (1_u32, 1_u32));

        let output = encoder.encode_text(&chars("ab"), table);

        assert_eq!(output, vec![0b0100_0000]);
    }
}
