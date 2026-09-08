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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines_single_field() {
        let cols: &[usize] = &[1];
        let res = parse_lines(cols, "this\tis\ttab\tseparated\tline", "\t").unwrap();
        let expected: Vec<Vec<&str>> = vec![Vec::from(["this"])];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_lines_multiple_fields() {
        let cols: &[usize] = &[1, 2];
        let res = parse_lines(cols, "this\tis\ttab\tseparated\tline", "\t").unwrap();
        let expected: Vec<Vec<&str>> = vec![Vec::from(["this", "is"])];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_lines_custom_delimiter() {
        let cols: &[usize] = &[1, 2];
        let res = parse_lines(cols, "this,is,tab,separated,line", ",").unwrap();
        let expected: Vec<Vec<&str>> = vec![Vec::from(["this", "is"])];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_lines_field_zero_returns_error() {
        let cols: &[usize] = &[0];
        let res = parse_lines(cols, "this,is,tab,separated,line", ",");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_lines_field_out_of_range_returns_error() {
        let cols: &[usize] = &[10];
        let res = parse_lines(cols, "this,is,tab,separated,line", ",");
        assert!(res.is_err());
    }
}
