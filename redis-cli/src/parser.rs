use crate::error::RedisError;

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

    fn parse_integer(&mut self) -> Result<i64, RedisError> {
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

        Ok(if is_negative { -number } else { number })
    }

    fn parse_simple_string(&mut self) -> Result<String, RedisError> {
        if self.peek() != Some(b'+') {
            return Err(RedisError::InvalidSimpleString(
                "Invalid starting token for simple string".to_string(),
            ));
        }

        self.pos += 1;
        let start = self.pos;

        while let Some(byte) = self.peek() {
            if self.peek() == Some(b'\r') {
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

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_valid_positive_integer() {
        let input = b":1000\r\n".to_vec();
        let expected: i64 = 1000;

        let mut parser = Parser::new(input);
        let output = parser.parse_integer().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_negative_integer() {
        let input = b":-1000\r\n".to_vec();
        let expected: i64 = -1000;

        let mut parser = Parser::new(input);
        let output = parser.parse_integer().unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parser_valid_simple_string() {
        let input = b"+Ok\r\n".to_vec();
        let expected: String = String::from("Ok");

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
}
