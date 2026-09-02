use std::collections::HashMap;

use crate::tree::{HuffmanNode, build_huffman_tree, create_pq};
use bitvec::{order::Msb0, slice::BitSlice, view::BitView};
use common::input_reader::read_bytes;
use std::io::Write;

#[derive(Default, Clone)]
pub struct Decoder<'a> {
    input_file_path: Option<&'a str>,
    output_file_path: String,
}

#[derive(Debug, PartialEq)]
struct Header {
    char_count: u32,
    total_char_count: u32,
    freq_map: HashMap<char, u32>,
    offset_end: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(input_file_path: Option<&'a str>, output_file_path: String) -> Self {
        Self {
            input_file_path,
            output_file_path,
        }
    }

    pub fn decode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_bytes: Vec<u8> = read_bytes(self.input_file_path)?;
        let header: Header = self.read_header(&raw_bytes)?;
        let mut pq = create_pq(&header.freq_map);
        let tree = build_huffman_tree(&mut pq)?;
        let text: String = self.decode_text(&header, &raw_bytes, &tree)?;
        self.write_output_file(&text)?;
        Ok(())
    }

    fn read_header(&mut self, raw_bytes: &[u8]) -> Result<Header, Box<dyn std::error::Error>> {
        let mut freq_map: HashMap<char, u32> = HashMap::new();
        let char_count_bytes: [u8; 4] = raw_bytes
            .get(0..4)
            .ok_or("header is too short")?
            .try_into()?;
        let char_count: u32 = u32::from_be_bytes(char_count_bytes);

        let total_char_count_bytes: [u8; 4] = raw_bytes
            .get(4..8)
            .ok_or("header is missing total char count")?
            .try_into()?;
        let total_char_count: u32 = u32::from_be_bytes(total_char_count_bytes);

        let mut curr_offset: usize = 8;

        for _ in 0..char_count {
            let char_bytes: [u8; 4] = raw_bytes
                .get(curr_offset..curr_offset + 4)
                .ok_or("Unable to read char for hashmap")?
                .try_into()?;
            let char_code: u32 = u32::from_be_bytes(char_bytes);
            let curr_char: char = char::from_u32(char_code)
                .ok_or(format!("Invalid character found {}", char_code))?;

            curr_offset += 4;
            let freq_bytes: [u8; 4] = raw_bytes
                .get(curr_offset..curr_offset + 4)
                .ok_or("Unable to read char freq for hashmap")?
                .try_into()?;
            let freq: u32 = u32::from_be_bytes(freq_bytes);
            freq_map.insert(curr_char, freq);

            curr_offset += 4;
        }

        Ok(Header {
            char_count,
            total_char_count,
            freq_map,
            offset_end: curr_offset,
        })
    }

    fn decode_text(
        &self,
        header: &Header,
        raw_bytes: &[u8],
        root: &HuffmanNode,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let content_bytes = &raw_bytes[header.offset_end..];
        let content_bits: &BitSlice<u8, Msb0> = content_bytes.view_bits::<Msb0>();
        let mut text: String = String::new();

        let mut char_count: u32 = 0;

        let mut trav_node: &HuffmanNode = root;
        for curr_bit in content_bits.iter() {
            if char_count >= header.total_char_count {
                break;
            }
            if trav_node.is_leaf {
                match trav_node.character {
                    Some(c) => {
                        text.push(c);
                        char_count += 1;
                    }
                    None => {
                        return Err("No character found in leaf node".into());
                    }
                };
                trav_node = root;
            }

            if curr_bit == true {
                trav_node = match trav_node.right_child.as_deref() {
                    Some(next_node) => next_node,
                    None => return Err("bitstream doesn't match tree shape".into()),
                };
            } else {
                trav_node = match trav_node.left_child.as_deref() {
                    Some(next_node) => next_node,
                    None => return Err("bitstream doesn't match tree shape".into()),
                };
            }
        }

        if trav_node.is_leaf {
            match trav_node.character {
                Some(c) => {
                    text.push(c);
                }
                None => {
                    return Err("No character found in leaf node".into());
                }
            };
        }

        Ok(text)
    }

    fn write_output_file(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(&self.output_file_path)?;
        Write::write_all(&mut file, text.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_header() {
        let mut decoder = Decoder::default();

        let mut raw_bytes: Vec<u8> = Vec::new();
        raw_bytes.extend_from_slice(&1_u32.to_be_bytes());
        raw_bytes.extend_from_slice(&4_u32.to_be_bytes());
        raw_bytes.extend_from_slice(&('a' as u32).to_be_bytes());
        raw_bytes.extend_from_slice(&4_u32.to_be_bytes());

        let output = decoder.read_header(&raw_bytes).unwrap();

        let mut expected_freq_map = HashMap::new();
        expected_freq_map.insert('a', 4_u32);

        let expected = Header {
            char_count: 1,
            total_char_count: 4,
            freq_map: expected_freq_map,
            offset_end: 16,
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn test_read_header_too_short() {
        let mut decoder = Decoder::default();
        let raw_bytes: Vec<u8> = vec![0, 0, 0];

        assert!(decoder.read_header(&raw_bytes).is_err());
    }

    #[test]
    fn test_decode_text() {
        // "aaaabbc" freqs (a:4, b:2, c:1) produce codes a=[1], b=[0,1], c=[0,0]
        let encoder = crate::encoder::Encoder::default();
        let freqs = encoder.get_frequencies(&"aaaabbc".chars().collect::<Vec<char>>());
        let mut pq = create_pq(&freqs);
        let tree = build_huffman_tree(&mut pq).unwrap();

        // "ac" -> bits [1,0,0] padded to one byte -> 0b1000_0000
        let raw_bytes: Vec<u8> = vec![0b1000_0000];

        let decoder = Decoder::default();
        let header = Header {
            char_count: 3,
            total_char_count: 2,
            freq_map: freqs,
            offset_end: 0,
        };

        let output = decoder.decode_text(&header, &raw_bytes, &tree).unwrap();

        assert_eq!(output, "ac");
    }

    #[test]
    fn test_write_output_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap().to_string();

        let decoder = Decoder::new(None, path.clone());
        decoder.write_output_file("hello world").unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "hello world");
    }
}
