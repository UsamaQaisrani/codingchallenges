use std::{char, collections::HashMap};

use crate::tree::{HuffmanNode, build_huffman_tree, create_pq};
use bitvec::prelude::*;
use common::input_reader::read_string;
use std::io::Write;

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
        let mut pq = create_pq(&freqs);
        let tree = build_huffman_tree(&mut pq)?;
        let lookup_table = self.generate_prefix_code_table(tree);
        self.write_encoded_file(&chars, &self.output_file_path, freqs, lookup_table)?;

        Ok(())
    }

    pub fn get_frequencies(&self, chars: &[char]) -> HashMap<char, u32> {
        let mut freq_map: HashMap<char, u32> = HashMap::new();

        for char in chars.iter() {
            let count = freq_map.entry(*char).or_insert(0_u32);
            *count += 1;
        }

        freq_map
    }

    fn generate_prefix_code_table(&self, node: HuffmanNode) -> HashMap<char, BitVec<u8, Msb0>> {
        let mut table = HashMap::new();
        let code = BitVec::new();
        self.traverse(&node, code, &mut table);
        table
    }

    fn traverse(
        &self,
        node: &HuffmanNode,
        code: BitVec<u8, Msb0>,
        table: &mut HashMap<char, BitVec<u8, Msb0>>,
    ) {
        if let Some(character) = node.character {
            table.insert(character, code);
            return;
        };

        if let Some(left) = &node.left_child {
            let mut left_code = code.clone();
            left_code.push(false);

            self.traverse(left, left_code, table);
        }

        if let Some(right) = &node.right_child {
            let mut right_code = code.clone();
            right_code.push(true);
            self.traverse(right, right_code, table);
        }
    }

    fn write_encoded_file(
        &self,
        chars: &[char],
        output_file_path: &str,
        freq_map: HashMap<char, u32>,
        lookup_table: HashMap<char, BitVec<u8, Msb0>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes: Vec<u8> = Vec::new();
        let header = self.create_header(chars.len() as u32, freq_map);
        let encoded_text = self.encode_text(chars, lookup_table);

        bytes.extend_from_slice(&header);
        bytes.extend_from_slice(&encoded_text);

        let mut file = std::fs::File::create(output_file_path)?;
        Write::write_all(&mut file, &bytes)?;

        Ok(())
    }

    fn create_header(&self, total_char_count: u32, table: HashMap<char, u32>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let map_length: u32 = table.len() as u32;

        // Add characters count
        bytes.extend_from_slice(&map_length.to_be_bytes());

        // Add total_char_count
        bytes.extend_from_slice(&total_char_count.to_be_bytes());

        // Add characters and their frequencies
        for (key, val) in table.iter() {
            let c: u32 = *key as u32;
            bytes.extend_from_slice(&c.to_be_bytes());
            bytes.extend_from_slice(&val.to_be_bytes());
        }

        bytes
    }

    fn encode_text(&self, chars: &[char], table: HashMap<char, BitVec<u8, Msb0>>) -> Vec<u8> {
        let mut bits = BitVec::<u8, Msb0>::new();

        for character in chars.iter() {
            bits.extend_from_bitslice(&table[character]);
        }

        bits.into_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{build_huffman_tree, create_pq};

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
    fn test_generate_prefix_code_table() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("aaaabbc"));
        let mut pq = create_pq(&freqs);
        let tree = build_huffman_tree(&mut pq).unwrap();
        let output = encoder.generate_prefix_code_table(tree);

        assert_eq!(output.get(&'a'), Some(&bitvec![u8, Msb0; 1]));
        assert_eq!(output.get(&'b'), Some(&bitvec![u8, Msb0; 0, 1]));
        assert_eq!(output.get(&'c'), Some(&bitvec![u8, Msb0; 0, 0]));
    }

    #[test]
    fn test_generate_prefix_code_table_single_symbol() {
        let encoder = Encoder::default();
        let freqs = encoder.get_frequencies(&chars("a"));
        let mut pq = create_pq(&freqs);
        let tree = build_huffman_tree(&mut pq).unwrap();
        let output = encoder.generate_prefix_code_table(tree);

        assert_eq!(output.get(&'a'), Some(&bitvec![u8, Msb0; 0]));
    }

    #[test]
    fn test_create_header() {
        let encoder = Encoder::default();
        let mut freqs = HashMap::new();
        freqs.insert('a', 4_u32);

        let output = encoder.create_header(1_u32, freqs);
        let mut expected: Vec<u8> = Vec::new();
        expected.extend_from_slice(&1_u32.to_be_bytes());
        expected.extend_from_slice(&1_u32.to_be_bytes());
        expected.extend_from_slice(&('a' as u32).to_be_bytes());
        expected.extend_from_slice(&4_u32.to_be_bytes());

        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_text() {
        let encoder = Encoder::default();
        let mut table = HashMap::new();
        table.insert('a', bitvec![u8, Msb0; 0]);
        table.insert('b', bitvec![u8, Msb0; 1]);

        let output = encoder.encode_text(&chars("ab"), table);

        assert_eq!(output, vec![0b0100_0000]);
    }
}
