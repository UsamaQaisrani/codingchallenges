use common::input_reader::{read_bytes, read_string};

pub fn count_bytes(file: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(read_bytes(file)?.len())
}

pub fn count_lines(file: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(read_string(file)?.lines().count())
}

pub fn count_words(file: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(read_string(file)?.split_whitespace().count())
}

pub fn count_chars(file: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(read_string(file)?.chars().count())
}
