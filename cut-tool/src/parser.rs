pub fn parse_lines<'a>(
    cols: &[usize],
    text: &'a str,
    delimiter: &'a str,
) -> Result<Vec<Vec<&'a str>>, Box<dyn std::error::Error>> {
    if cols.contains(&0) {
        return Err("Field numbers start at 1".into());
    }

    let mut col_vec: Vec<Vec<&str>> = Vec::new();

    for lines in text.lines() {
        let mut curr_list: Vec<&str> = Vec::new();
        for col in cols.iter() {
            let word = lines
                .split(delimiter)
                .nth(col - 1)
                .ok_or("Unable to split on separator")?;
            curr_list.push(word);
        }
        col_vec.push(curr_list);
    }

    Ok(col_vec)
}
