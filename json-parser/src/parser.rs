use crate::lexer::Token;
use core::fmt;
use miette::{Result, miette};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Null => write!(f, "Null"),
            JsonValue::Bool(b) => write!(f, "{b}"),
            JsonValue::Number(n) => write!(f, "{n}"),
            JsonValue::String(s) => write!(f, "{s}"),
            JsonValue::Array(v) => write!(f, "{v:?}"),
            JsonValue::Object(m) => write!(f, "{m:?}"),
        }
    }
}

pub struct Parser {
    pos: usize,
    pub input: Vec<Token>,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self { pos: 0, input }
    }

    fn peek(&self) -> Option<Token> {
        if self.pos >= self.input.len() {
            return None;
        }
        Some(self.input[self.pos].clone())
    }

    fn expected(&mut self, token: Token) -> Result<bool> {
        let peek_char: Token = self
            .peek()
            .ok_or_else(|| miette!("Unexpected end of input"))?;
        if peek_char == token {
            self.advance();
            Ok(true)
        } else {
            Err(miette!(
                "Unexpected token: {}, expected: {}",
                self.input[self.pos],
                token
            ))
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        let result = self.parse_value()?;
        Ok(result)
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        let token: Token = self.input[self.pos].clone();
        match token {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::Number(n) => {
                self.advance();
                Ok(JsonValue::Number(n))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(JsonValue::Bool(b))
            }
            Token::Null => {
                self.advance();
                Ok(JsonValue::Null)
            }
            Token::String(s) => {
                self.advance();
                Ok(JsonValue::String(s))
            }
            _ => Err(miette!("Invalid Json")),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        self.advance();
        match self.peek() {
            Some(token) => match token {
                Token::RightBrace => {
                    self.advance();
                    Ok(JsonValue::Object(HashMap::new()))
                }
                Token::String(key) => {
                    self.advance();
                    let value = self.parse_pair()?;
                    let mut map: HashMap<String, JsonValue> = HashMap::new();
                    map.insert(key, value);
                    while let Some(peek_token) = self.peek()
                        && peek_token == Token::Comma
                    {
                        self.advance();
                        let key = self.input[self.pos].to_string();
                        self.advance();
                        let value = self.parse_pair()?;
                        map.insert(key, value);
                    }

                    self.advance();
                    Ok(JsonValue::Object(map))
                }
                _ => Err(miette!("Unexpected token in object")),
            },
            None => Err(miette!("Invalid Json, unexpected end of json")),
        }
    }

    fn parse_pair(&mut self) -> Result<JsonValue> {
        let _valid: bool = self.expected(Token::Colon)?;
        let value: JsonValue = self.parse_value()?;
        Ok(value)
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        let mut res_array: Vec<JsonValue> = Vec::new();
        self.advance();

        if let Some(peek_token) = self.peek()
            && peek_token == Token::RightBracket
        {
            self.advance();
            return Ok(JsonValue::Array(res_array));
        }

        let val = self.parse_value()?;
        res_array.push(val);

        while let Some(peek_token) = self.peek()
            && peek_token == Token::Comma
        {
            self.advance();
            let val = self.parse_value()?;
            res_array.push(val);
        }
        self.advance();
        Ok(JsonValue::Array(res_array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_empty_object() {
        let input: &str = "{}";
        let expected = JsonValue::Object(HashMap::new());

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_single_pair() {
        let input: &str = "{\"key\":\"value\"}";
        let mut map: HashMap<String, JsonValue> = HashMap::new();
        map.insert(
            String::from("key"),
            JsonValue::String(String::from("value")),
        );
        let expected = JsonValue::Object(map);

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_multiple_pairs() {
        let input: &str = "{\"a\":\"1\", \"b\":\"2\"}";
        let mut map: HashMap<String, JsonValue> = HashMap::new();
        map.insert(String::from("a"), JsonValue::String(String::from("1")));
        map.insert(String::from("b"), JsonValue::String(String::from("2")));
        let expected = JsonValue::Object(map);

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_all_value_types() {
        let input: &str = "{\"s\":\"x\", \"n\":1, \"b\":true, \"z\":null}";
        let mut map: HashMap<String, JsonValue> = HashMap::new();
        map.insert(String::from("s"), JsonValue::String(String::from("x")));
        map.insert(String::from("n"), JsonValue::Number(1_f64));
        map.insert(String::from("b"), JsonValue::Bool(true));
        map.insert(String::from("z"), JsonValue::Null);
        let expected = JsonValue::Object(map);

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_nested_object_and_array() {
        let input: &str = "{\"o\":{\"inner\":1},\"a\":[1,2]}";

        let mut inner_map: HashMap<String, JsonValue> = HashMap::new();
        inner_map.insert(String::from("inner"), JsonValue::Number(1.0));

        let mut map: HashMap<String, JsonValue> = HashMap::new();
        map.insert(String::from("o"), JsonValue::Object(inner_map));
        map.insert(
            String::from("a"),
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        );
        let expected = JsonValue::Object(map);

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_empty_array() {
        let input: &str = "[]";
        let expected = JsonValue::Array(Vec::new());

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse().unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_invalid_json_returns_err() {
        let input: &str = "{\"a\":}";

        let mut lexer = Lexer::new(input);
        let tokens = lexer.process_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let output = parser.parse();

        assert!(output.is_err());
    }
}
