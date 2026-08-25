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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_count_bytes_ascii_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello").unwrap();

        let result = count_bytes(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 5);
    }

    #[test]
    fn test_count_bytes_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let result = count_bytes(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_bytes_multibyte_utf8() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "héllo").unwrap();

        let result = count_bytes(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 6);
    }

    #[test]
    fn test_count_lines_with_trailing_newline() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "hello").unwrap();

        let result = count_lines(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_lines_without_trailing_newline() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello").unwrap();

        let result = count_lines(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_lines_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();

        let result = count_lines(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_words_multiple_spaces_between_words() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello   world").unwrap();

        let result = count_words(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_words_leading_and_trailing_whitepsace() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "  hello   world   ").unwrap();

        let result = count_words(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_words_tabs_and_newlines_as_separators() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello  \nworld").unwrap();

        let result = count_words(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_words_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();

        let result = count_words(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_chars_ascii_matches_byte_count() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello").unwrap();

        let byte_result = count_bytes(Some(tmp.path().to_str().unwrap())).unwrap();
        let char_result = count_chars(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(byte_result, char_result);
        assert_eq!(char_result, 5);
    }

    #[test]
    fn test_count_chars_multibyte_utf8_differs_from_byte_count() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "héllo").unwrap();

        let byte_result = count_bytes(Some(tmp.path().to_str().unwrap())).unwrap();
        let char_result = count_chars(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(byte_result, 6);
        assert_eq!(char_result, 5);
        assert_ne!(byte_result, char_result);
    }

    #[test]
    fn test_count_chars_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();

        let result = count_chars(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, 0);
    }
}
