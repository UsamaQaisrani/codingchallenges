use crate::error::RedisError;
use crate::parser::RespValue;

pub struct Encoder {
    input: RespValue,
}

impl Encoder {
    pub fn new(input: RespValue) -> Self {
        Encoder { input }
    }

    pub fn encode(&self) -> Result<Vec<u8>, RedisError> {
        let val = match &self.input {
            RespValue::SimpleString(s) => Self::encode_simple_string(s)?,
            _ => {
                return Err(RedisError::InvalidType);
            }
        };
        Ok(val)
    }

    fn encode_simple_string(message: &str) -> Result<Vec<u8>, RedisError> {
        if message.contains("\r") || message.contains("\n") {
            return Err(RedisError::InvalidSimpleString(
                "Simple string cannot contain CRLF".into(),
            ));
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b'+');
        buf.extend_from_slice(message.as_bytes());
        buf.extend_from_slice(b"\r\n");
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string_ok() {
        let input = "OK";
        let output = Encoder::encode_simple_string(input).unwrap();
        let expected = b"+OK\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_simple_string_empty() {
        let input = "";
        let output = Encoder::encode_simple_string(input).unwrap();
        let expected = b"+\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_simple_string_rejects_cr() {
        let input = "\r";
        let output = Encoder::encode_simple_string(input).unwrap_err();
        assert!(output.to_string().contains("CRLF"));
    }

    #[test]
    fn test_encode_simple_string_rejects_lf() {
        let input = "\n";
        let output = Encoder::encode_simple_string(input).unwrap_err();
        assert!(output.to_string().contains("CRLF"));
    }

    #[test]
    fn test_encode_simple_string_utf8() {
        let input = "é";
        let output = Encoder::encode_simple_string(input).unwrap();
        let expected = b"+\xC3\xA9\r\n";
        assert_eq!(output, expected);
    }
}
