use common::input_reader::read_string;

pub fn sort(path: Option<&str>, dedup: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut words: Vec<String> = read_string(path)?
        .lines()
        .map(|line| line.to_string())
        .collect();
    words.sort();

    if dedup {
        words.dedup();
    }

    Ok(words)
}
