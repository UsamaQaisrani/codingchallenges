use std::collections::HashMap;

use common::input_reader::read_bytes;

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

    fn decode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_bytes: Vec<u8> = read_bytes(self.input_file_path)?;
        let _header: Header = self.read_header(&raw_bytes)?;
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
}
