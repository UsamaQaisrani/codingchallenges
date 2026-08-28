use crate::lexer::Token;
use miette::{Result, miette};
use std::collections::HashMap;

pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
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
