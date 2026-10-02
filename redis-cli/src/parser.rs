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

        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err(RedisError::InvalidInteger(
                "Invalid end of integer".to_string(),
            ));
        }

        if self.pos == start {
            return Err(RedisError::InvalidInteger(
                "Empty bytes in integer while parsing".to_string(),
            ));
        }

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
            if byte == b'\r' {
                break;
            }

            self.pos += 1;
        }

        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err(RedisError::InvalidSimpleString(
                "Invalid termination for simple string".to_string(),
            ));
        }

        if start == self.pos {
            return Err(RedisError::InvalidSimpleString(
                "Empty string while parsing".to_string(),
            ));
        }

        let res: String = std::str::from_utf8(&self.input[start..self.pos])
            .map_err(|e| RedisError::InvalidSimpleString(format!("{}", e)))?
            .to_string();

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
            if byte == b'\r' {
                break;
            }

            self.pos += 1;
        }

        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err(RedisError::InvalidSimpleError(
                "Invalid termination for simple error".to_string(),
            ));
        }

        if start == self.pos {
            return Err(RedisError::InvalidSimpleError(
                "Empty error string while parsing".to_string(),
            ));
        }

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

            if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
                return Err(RedisError::InvalidBulkStringError(
                    "Invalid end of null bulk string".to_string(),
                ));
            }

            self.pos += 2;

            return Ok(RespValue::BulkString(None));
        }

        let start = self.pos;

        // Get length of BulkString
        while let Some(byte) = self.peek() {
            if byte == b'\r' {
                break;
            }

            self.pos += 1;
        }

        if self.input.get(self.pos..self.pos + 2) != Some(b"\r\n") {
            return Err(RedisError::InvalidBulkStringError(
                "Invalid end of bulk string length, expected CRLF".to_string(),
            ));
        }

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

        let end = self.pos + length;
        let res: Vec<u8> = self.input[self.pos..end].to_vec();

        Ok(RespValue::BulkString(Some(res)))
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
    fn test_parse_empty_bulk_string() {
        let input = b"$0\r\n\r\n".to_vec();
        let expected: RespValue = RespValue::BulkString(Some(b"".to_vec()));

        let mut parser = Parser::new(input);
        let output = parser.parse_bulk_string().unwrap();
        assert_eq!(output, expected);
    }
}
