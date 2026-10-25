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
            RespValue::Integer(n) => Self::encode_integer(*n),
            RespValue::BulkString(s) => Self::encode_bulk_string(s),
            RespValue::Error(e) => Self::encode_simple_error(e)?,
            RespValue::Array(l) => Self::encode_array(l)?,
        };
        Ok(val)
    }

    fn encode_simple_string(message: &str) -> Result<Vec<u8>, RedisError> {
        if message.contains("\r") || message.contains("\n") {
            return Err(RedisError::InvalidSimpleString(
                "Simple string cannot contain only CR or LF".into(),
            ));
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b'+');
        buf.extend_from_slice(message.as_bytes());
        buf.extend_from_slice(b"\r\n");
        Ok(buf)
    }

    fn encode_integer(num: i64) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b':');
        buf.extend_from_slice(num.to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }

    fn encode_bulk_string(message: &Option<Vec<u8>>) -> Vec<u8> {
        match message {
            None => b"$-1\r\n".to_vec(),
            Some(message) => {
                let length = message.len();
                let mut buf: Vec<u8> = Vec::new();

                buf.push(b'$');
                buf.extend_from_slice(length.to_string().as_bytes());
                buf.extend_from_slice(b"\r\n");
                buf.extend_from_slice(message);
                buf.extend_from_slice(b"\r\n");

                buf
            }
        }
    }

    fn encode_simple_error(message: &str) -> Result<Vec<u8>, RedisError> {
        if message.contains("\r") || message.contains("\n") {
            return Err(RedisError::InvalidSimpleError(
                "Simple error cannot contain only CR or LF".into(),
            ));
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.push(b'-');
        buf.extend_from_slice(message.as_bytes());
        buf.extend_from_slice(b"\r\n");
        Ok(buf)
    }

    fn encode_array(input: &Option<Vec<RespValue>>) -> Result<Vec<u8>, RedisError> {
        match input {
            None => Ok(b"*-1\r\n".to_vec()),
            Some(list) => {
                let mut buf: Vec<u8> = Vec::new();
                buf.push(b'*');
                buf.extend_from_slice(list.len().to_string().as_bytes());
                buf.extend_from_slice(b"\r\n");
                for item in list {
                    let encoder = Encoder::new(item.clone());
                    let encoded_item = encoder.encode()?;
                    buf.extend_from_slice(&encoded_item);
                }

                Ok(buf)
            }
        }
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
        assert!(output.to_string().contains("CR"));
    }

    #[test]
    fn test_encode_simple_string_rejects_lf() {
        let input = "\n";
        let output = Encoder::encode_simple_string(input).unwrap_err();
        assert!(output.to_string().contains("LF"));
    }

    #[test]
    fn test_encode_simple_string_utf8() {
        let input = "é";
        let output = Encoder::encode_simple_string(input).unwrap();
        let expected = b"+\xC3\xA9\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_integer_positive_has_no_sign() {
        let input = 1000;
        let output = Encoder::encode_integer(input);
        let expected = b":1000\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_integer_negative() {
        let input = -1000;
        let output = Encoder::encode_integer(input);
        let expected = b":-1000\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_integer_zero() {
        let input = 0;
        let output = Encoder::encode_integer(input);
        let expected = b":0\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_integer_i64_min() {
        let input = i64::MIN;
        let output = Encoder::encode_integer(input);
        let expected = b":-9223372036854775808\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_bulk_string_ok() {
        let input = b"hello".to_vec();
        let output = Encoder::encode_bulk_string(&Some(input));
        let expected = b"$5\r\nhello\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_bulk_string_empty() {
        let input = b"".to_vec();
        let output = Encoder::encode_bulk_string(&Some(input));
        let expected = b"$0\r\n\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_bulk_string_contains_crlf() {
        let input = b"\r\n".to_vec();
        let output = Encoder::encode_bulk_string(&Some(input));
        let expected = b"$2\r\n\r\n\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_simple_error_ok() {
        let input = "Error message";
        let output = Encoder::encode_simple_error(input).unwrap();
        let expected = b"-Error message\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_simple_error_empty() {
        let input = "";
        let output = Encoder::encode_simple_error(input).unwrap();
        let expected = b"-\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_simple_error_rejects_cr() {
        let input = "\r";
        let output = Encoder::encode_simple_error(input).unwrap_err();
        assert!(output.to_string().contains("CR"));
    }

    #[test]
    fn test_encode_simple_error_rejects_lf() {
        let input = "\n";
        let output = Encoder::encode_simple_error(input).unwrap_err();
        assert!(output.to_string().contains("LF"));
    }

    #[test]
    fn test_encode_array_empty() {
        let input: Vec<RespValue> = Vec::new();
        let output = Encoder::encode_array(&Some(input)).unwrap();
        let expected = b"*0\r\n".to_vec();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_array_null() {
        let output = Encoder::encode_array(&None).unwrap();
        let expected = b"*-1\r\n".to_vec();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_array_of_bulk_strings() {
        let input: Vec<RespValue> = Vec::from([
            RespValue::BulkString(Some("SET".into())),
            RespValue::BulkString(Some("k".into())),
            RespValue::BulkString(Some("v".into())),
        ]);
        let output = Encoder::encode_array(&Some(input)).unwrap();
        let expected = b"*3\r\n$3\r\nSET\r\n$1\r\nk\r\n$1\r\nv\r\n".to_vec();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_array_mixed_types() {
        let input: Vec<RespValue> = Vec::from([
            RespValue::BulkString(Some("SET".into())),
            RespValue::Integer(1),
        ]);
        let output = Encoder::encode_array(&Some(input)).unwrap();
        let expected = b"*2\r\n$3\r\nSET\r\n:1\r\n".to_vec();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_array_nested() {
        let input: Vec<RespValue> = Vec::from([
            RespValue::BulkString(Some("SET".into())),
            RespValue::Array(Some(Vec::from([RespValue::BulkString(Some(
                "INNER".into(),
            ))]))),
        ]);
        let output = Encoder::encode_array(&Some(input)).unwrap();
        let expected = b"*2\r\n$3\r\nSET\r\n*1\r\n$5\r\nINNER\r\n".to_vec();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_encode_array_propagates_error() {
        let input: Vec<RespValue> = Vec::from([RespValue::SimpleString("\r".into())]);
        let output = Encoder::encode_array(&Some(input)).unwrap_err();
        assert!(output.to_string().contains("CR"));
    }
}
