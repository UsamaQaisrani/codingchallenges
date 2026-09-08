pub fn parse_lines<'a>(
    n: usize,
    text: &'a str,
    delimiter: &'a str,
) -> Result<Vec<&'a str>, Box<dyn std::error::Error>> {
    if n == 0 {
        return Err("Field numbers start at 1".into());
    }

    let mut col_vec: Vec<&str> = Vec::new();

    for lines in text.lines() {
        let word = lines
            .split(delimiter)
            .nth(n - 1)
            .ok_or("Unable to split on separator")?;
        col_vec.push(word);
    }
    Ok(col_vec)
}
