pub fn parse_lines(n: usize, text: &str) -> Result<Vec<&str>, Box<dyn std::error::Error>> {
    if n == 0 {
        return Err("Field numbers start at 1".into());
    }

    let mut col_vec: Vec<&str> = Vec::new();

    for lines in text.lines() {
        let word = lines
            .split("\t")
            .nth(n - 1)
            .ok_or("Unable to split on separator")?;
        col_vec.push(word);
    }
    Ok(col_vec)
}
