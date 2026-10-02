use crate::error::RedisError;

#[derive(Debug, PartialEq)]
enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<Vec<u8>>),
    Array(Option<Vec<RespValue>>),
}

pub struct Parser {
    pos: usize,
    input: Vec<u8>,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        Self { pos: 0, input }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn parse_integer(&mut self) -> Result<RespValue, RedisError> {
        if self.peek() != Some(b':') {
            return Err(RedisError::InvalidInteger(
                "Invalid format for integer: missing ':'".to_string(),
            ));
        }

        self.pos += 1;
        let is_negative: bool = if self.peek() == Some(b'-') {
            self.pos += 1;
            true
        } else {
            false
        };

        let start = self.pos;

        while let Some(byte) = self.peek() {
            if byte == b'\r' {
                break;
            }

            if !byte.is_ascii_digit() {
                return Err(RedisError::InvalidInteger(format!(
                    "Invalid format: expected integer found {}",
                    byte
                )));
            }

            self.pos += 1;
        }

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidInteger)?;

        self.check_empty_bytes_data(&start)
            .map_err(RedisError::InvalidInteger)?;

        let number = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidInteger(format!("{}", e)))?
            .parse::<i64>()
            .map_err(|e| RedisError::InvalidInteger(format!("{}", e)))?;

        self.pos += 2;

        Ok(if is_negative {
            RespValue::Integer(-number)
        } else {
            RespValue::Integer(number)
        })
    }

    fn parse_simple_string(&mut self) -> Result<RespValue, RedisError> {
        if self.peek() != Some(b'+') {
            return Err(RedisError::InvalidSimpleString(
                "Invalid starting token for simple string".to_string(),
            ));
        }

        self.pos += 1;
        let start = self.pos;

        while let Some(byte) = self.peek() {
            if byte == b'\r' || byte == b'\n' {
                break;
            }

            self.pos += 1;
        }

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidSimpleString)?;

        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err(RedisError::InvalidSimpleString(
                "Invalid termination for simple string".to_string(),
            ));
        }

        let res: String = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidSimpleString(format!("{}", e)))?
            .to_string();

        self.pos += 2;

        Ok(RespValue::SimpleString(res))
    }

    fn parse_simple_error(&mut self) -> Result<RespValue, RedisError> {
        if self.peek() != Some(b'-') {
            return Err(RedisError::InvalidSimpleError(
                "Invalid starting token for Error".to_string(),
            ));
        }

        self.pos += 1;

        let start = self.pos;

        while let Some(byte) = self.peek() {
            if byte == b'\r' || byte == b'\n' {
                break;
            }

            self.pos += 1;
        }

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidSimpleError)?;

        let error: String = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidSimpleError(format!("{}", e)))?
            .to_string();

        self.pos += 2;

        Ok(RespValue::Error(error))
    }

    fn parse_bulk_string(&mut self) -> Result<RespValue, RedisError> {
        if self.peek() != Some(b'$') {
            return Err(RedisError::InvalidBulkStringError(
                "Invalid starting token for bulk string".to_string(),
            ));
        }

        self.pos += 1;

        // Check Null BulkString
        if self.peek() == Some(b'-') {
            self.pos += 1;

            if self.peek() != Some(b'1') {
                return Err(RedisError::InvalidBulkStringError(
                    "Invalid length of null bulk string".to_string(),
                ));
            }

            self.pos += 1;

            self.check_valid_end_of_content_crlf()
                .map_err(RedisError::InvalidBulkStringError)?;

            self.pos += 2;

            return Ok(RespValue::BulkString(None));
        }

        let start = self.pos;

        // Get length of BulkString
        while let Some(byte) = self.peek() {
            if byte == b'\r' {
                break;
            }

            if !byte.is_ascii_digit() {
                return Err(RedisError::InvalidBulkStringError(
                    "Inavlid token in bulk string length".to_string(),
                ));
            }

            self.pos += 1;
        }

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidBulkStringError)?;

        self.check_empty_bytes_data(&start)
            .map_err(RedisError::InvalidBulkStringError)?;

        if start == self.pos {
            return Err(RedisError::InvalidBulkStringError(
                "Empty length of bulk string".to_string(),
            ));
        }

        let length = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidBulkStringError(format!("{}", e)))?
            .parse::<usize>()
            .map_err(|e| RedisError::InvalidBulkStringError(format!("{}", e)))?;

        self.pos += 2;

        let end = match length.checked_add(self.pos) {
            Some(end) => end,
            None => {
                return Err(RedisError::InvalidBulkStringError(
                    "Invalid length provided, length overflowed".to_string(),
                ));
            }
        };

        if self.input.get(self.pos..end).is_none() {
            return Err(RedisError::InvalidBulkStringError(
                "Bulk string is shorter than declared length".to_string(),
            ));
        }

        let res = self.input[self.pos..end].to_vec();

        self.pos = end;

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidBulkStringError)?;

        self.pos += 2;

        Ok(RespValue::BulkString(Some(res)))
    }

    fn parse_array(&mut self) -> Result<RespValue, RedisError> {
        if self.peek() != Some(b'*') {
            return Err(RedisError::InvalidArrayError(
                "Invalid starting token for array".to_string(),
            ));
        }

        self.pos += 1;

        // Check Null Array
        if self.peek() == Some(b'-') {
            self.pos += 1;

            if self.peek() != Some(b'1') {
                return Err(RedisError::InvalidArrayError(
                    "Invalid length of null array".to_string(),
                ));
            }

            self.pos += 1;

            self.check_valid_end_of_content_crlf()
                .map_err(RedisError::InvalidArrayError)?;

            self.pos += 2;

            return Ok(RespValue::Array(None));
        }

        let start = self.pos;

        // Get Array Length
        while let Some(byte) = self.peek() {
            if byte == b'\r' {
                break;
            }

            if !byte.is_ascii_digit() {
                return Err(RedisError::InvalidArrayError(
                    "Inavlid token in array length".to_string(),
                ));
            }

            self.pos += 1;
        }

        self.check_valid_end_of_content_crlf()
            .map_err(RedisError::InvalidArrayError)?;

        self.check_empty_bytes_data(&start)
            .map_err(RedisError::InvalidArrayError)?;

        let array_length: u64 = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidArrayError(format!("{}", e)))?
            .parse::<u64>()
            .map_err(|e| RedisError::InvalidArrayError(format!("{}", e)))?;

        self.pos += 2;

        let mut array: Vec<RespValue> = Vec::new();

        for _ in 0..array_length {
            let res = match self.peek() {
                Some(b'+') => self.parse_simple_string()?,
                Some(b'-') => self.parse_simple_error()?,
                Some(b':') => self.parse_integer()?,
                Some(b'$') => self.parse_bulk_string()?,
                Some(b'*') => self.parse_array()?,
                _ => {
                    return Err(RedisError::InvalidArrayError(
                        "Invalid token found in array item".to_string(),
                    ));
                }
            };
            array.push(res);
        }

        Ok(RespValue::Array(Some(array)))
    }

    fn check_valid_end_of_content_crlf(&self) -> Result<(), String> {
        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err("Invalid end of length for array".to_string());
        }
        Ok(())
    }

    fn check_empty_bytes_data(&self, start: &usize) -> Result<(), String> {
        if start == &self.pos {
            return Err("Empty data bytes".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_valid_positive_integer() {
        let input = b":1000\r\n".to_vec();
        let expected: RespValue = RespValue::Integer(1000);

        let mut parser = Parser::new(input);
        let output = parser.parse_integer().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_negative_integer() {
        let input = b":-1000\r\n".to_vec();
        let expected: RespValue = RespValue::Integer(-1000);

        let mut parser = Parser::new(input);
        let output = parser.parse_integer().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_simple_string() {
        let input = b"+Ok\r\n".to_vec();
        let expected: RespValue = RespValue::SimpleString(String::from("Ok"));

        let mut parser = Parser::new(input);
        let output = parser.parse_simple_string().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_invalid_simple_string() {
        let input = b"+Ok\n".to_vec();

        let mut parser = Parser::new(input);
        let output = parser.parse_simple_string();
        assert!(output.is_err());
    }

    #[test]
    fn test_parser_valid_simple_error() {
        let input = b"-Error Message\r\n".to_vec();
        let expected: RespValue = RespValue::Error(String::from("Error Message"));

        let mut parser = Parser::new(input);
        let output = parser.parse_simple_error().unwrap();
        assert_eq!(output, expected);
    }
    #[test]
    fn test_parser_invalid_simple_error() {
        let input = b"-Error Message\n".to_vec();

        let mut parser = Parser::new(input);
        let output = parser.parse_simple_error();
        assert!(output.is_err());
    }

    #[test]
    fn test_parser_valid_bulk_string() {
        let input = b"$5\r\nhello\r\n".to_vec();
        let expected: RespValue = RespValue::BulkString(Some(b"hello".to_vec()));

        let mut parser = Parser::new(input);
        let output = parser.parse_bulk_string().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_null_bulk_string() {
        let input = b"$-1\r\n".to_vec();
        let expected: RespValue = RespValue::BulkString(None);

        let mut parser = Parser::new(input);
        let output = parser.parse_bulk_string().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_empty_bulk_string() {
        let input = b"$0\r\n\r\n".to_vec();
        let expected: RespValue = RespValue::BulkString(Some(b"".to_vec()));

        let mut parser = Parser::new(input);
        let output = parser.parse_bulk_string().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_array_with_single_item() {
        let input = b"*1\r\n$5\r\nhello\r\n".to_vec();
        let expected: RespValue =
            RespValue::Array(Some(vec![RespValue::BulkString(Some(b"hello".to_vec()))]));

        let mut parser = Parser::new(input);
        let output = parser.parse_array().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_array_with_multiple_items() {
        let input = b"*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n".to_vec();
        let expected: RespValue = RespValue::Array(Some(vec![
            RespValue::BulkString(Some(b"hello".to_vec())),
            RespValue::BulkString(Some(b"world".to_vec())),
        ]));

        let mut parser = Parser::new(input);
        let output = parser.parse_array().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_empty_array() {
        let input = b"*0\r\n".to_vec();
        let expected: RespValue = RespValue::Array(Some(vec![]));

        let mut parser = Parser::new(input);
        let output = parser.parse_array().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_null_array() {
        let input = b"*-1\r\n".to_vec();
        let expected: RespValue = RespValue::Array(None);

        let mut parser = Parser::new(input);
        let output = parser.parse_array().unwrap();
        assert_eq!(output, expected);
    }
}
