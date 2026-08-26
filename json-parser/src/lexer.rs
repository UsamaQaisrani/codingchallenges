#[derive(Clone)]
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
}
