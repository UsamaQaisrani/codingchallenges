use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub input: Vec<char>,
    pos: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::String(s) => write!(f, "{s}"),
            Token::Number(n) => write!(f, "{n}"),
            Token::Bool(b) => write!(f, "{b}"),
            Token::Null => write!(f, "Null"),
            Token::Comma => write!(f, ","),
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            tokens: Vec::new(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            return None;
        }
        Some(self.input[self.pos])
    }

    fn skip_whitespace(&mut self) {
        while let Some(peek_char) = self.peek()
            && peek_char == ' '
        {
            self.pos += 1;
        }
    }

    pub fn process_tokens(&mut self) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }
            match self.input[self.pos] {
                '[' => self.tokens.push(Token::LeftBracket),
                ']' => self.tokens.push(Token::RightBracket),
                '{' => self.tokens.push(Token::LeftBrace),
                '}' => self.tokens.push(Token::RightBrace),
                ',' => self.tokens.push(Token::Comma),
                ':' => self.tokens.push(Token::Colon),
                'f' | 't' | 'n' => {
                    let literal = self.process_literal()?;
                    self.tokens.push(literal);
                    continue;
                }
                '"' => {
                    let string_result = self.process_string();
                    self.tokens.push(Token::String(string_result));
                }
                c if c.is_ascii_digit() => {
                    let num = self.process_number()?;
                    self.tokens.push(Token::Number(num));
                    continue;
                }
                _ => return Err("Invalid json".into()),
            }
            self.pos += 1;
        }
        Ok(self.tokens.clone())
    }

    fn process_string(&mut self) -> String {
        self.pos += 1;
        let start_pos = self.pos;
        while let Some(peek_char) = self.peek()
            && peek_char != '"'
        {
            self.pos += 1;
        }

        self.input[start_pos..self.pos].iter().collect()
    }

    fn process_number(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let mut num_collection: Vec<char> = Vec::new();
        while let Some(peek_char) = self.peek()
            && peek_char.is_ascii_digit()
        {
            num_collection.push(peek_char);
            self.pos += 1;
        }
        let num: f64 = num_collection.iter().collect::<String>().parse()?;
        Ok(num)
    }

    fn process_literal(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        if self.input[self.pos..].starts_with(&['t', 'r', 'u', 'e']) {
            self.pos += 4;
            Ok(Token::Bool(true))
        } else if self.input[self.pos..].starts_with(&['f', 'a', 'l', 's', 'e']) {
            self.pos += 5;
            Ok(Token::Bool(false))
        } else if self.input[self.pos..].starts_with(&['n', 'u', 'l', 'l']) {
            self.pos += 4;
            Ok(Token::Null)
        } else {
            Err("Invalid JSON literal".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_tokens_structural_chars() {
        let input = "{}[],:";
        let output = Lexer::new(input).process_tokens().unwrap();
        let expected: Vec<Token> = vec![
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Comma,
            Token::Colon,
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_process_token_strings() {
        let input: &str = "{\"key\":\"value\"}";
        let output = Lexer::new(input).process_tokens().unwrap();
        let expected: Vec<Token> = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::RightBrace,
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_process_tokens_number() {
        let input: &str = "123";
        let output = Lexer::new(input).process_tokens().unwrap();
        let expected: Vec<Token> = vec![Token::Number(123_f64)];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_process_token_literals() {
        let input: &str = "[true, false, null]";
        let output = Lexer::new(input).process_tokens().unwrap();
        let expected: Vec<Token> = vec![
            Token::LeftBracket,
            Token::Bool(true),
            Token::Comma,
            Token::Bool(false),
            Token::Comma,
            Token::Null,
            Token::RightBracket,
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_process_tokens_trailing_whitespace() {
        let input = "{} ";
        let output = Lexer::new(input).process_tokens().unwrap();
        let expected: Vec<Token> = vec![Token::LeftBrace, Token::RightBrace];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_process_tokens_invalid_character() {
        let input = "!";
        let output = Lexer::new(input).process_tokens();

        assert!(output.is_err());
    }
}
