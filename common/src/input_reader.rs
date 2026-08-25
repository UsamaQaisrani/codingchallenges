use std::{
    fs,
    io::{self, Read},
};

fn read_all_bytes<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn read_bytes(file_path: Option<&str>) -> io::Result<Vec<u8>> {
    match file_path {
        Some(file_path) => read_all_bytes(fs::File::open(file_path)?),
        None => read_all_bytes(io::stdin()),
    }
}

fn read_all_string<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_string(file_path: Option<&str>) -> io::Result<String> {
    match file_path {
        Some(file_path) => read_all_string(fs::File::open(file_path)?),
        None => read_all_string(io::stdin()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_bytes_from_existing_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        tmp.write_all(b"hello").unwrap();

        let result = read_bytes(Some(tmp.path().to_str().unwrap())).unwrap();

        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_read_bytes_missing_file_returns_err() {
        let result = read_bytes(Some(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_bytes_from_stdin() {
        let fake_stdin = io::Cursor::new(b"hello from stdin");
        let result = read_all_bytes(fake_stdin).unwrap();
        assert_eq!(result, b"hello from stdin");
    }

    #[test]
    fn test_read_bytes_from_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let result = read_bytes(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, b"");
    }

    #[test]
    fn test_read_string_from_existing_file() {
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        write!(tmp, "hello").unwrap();

        let result = read_string(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_read_string_from_missing_file_returns_err() {
        let result = read_string(Some(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_string_from_stdin() {
        let fake_stdin = io::Cursor::new("hello");
        let result = read_all_string(fake_stdin).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_read_string_from_empty_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let result = read_string(Some(tmp.path().to_str().unwrap())).unwrap();
        assert_eq!(result, "");
    }
}
