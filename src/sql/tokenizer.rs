use super::token::Token;
use phf::phf_map;
use std::fmt;

pub static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    // Keywords
    "SELECT" => Token::Select,
    "FROM"   => Token::From,
    "WHERE"  => Token::Where,
    "INSERT" => Token::Insert,
    "INTO"   => Token::Into,
    "VALUES" => Token::Values,
    "UPDATE" => Token::Update,
    "SET"    => Token::Set,
    "DELETE" => Token::Delete,
    "CREATE" => Token::Create,
    "TABLE"  => Token::Table,
    "DROP"   => Token::Drop,
    "AND"    => Token::And,
    "OR"     => Token::Or,
    "NOT"    => Token::Not,
    "NULL"   => Token::Null,
    "IS"     => Token::Is,

    // Punctuation
    ","      => Token::Comma,
    "."      => Token::Dot,
    "("      => Token::LParen,
    ")"      => Token::RParen,
    ";"      => Token::Semicolon,
    "*"      => Token::Asterisk,

    // Special
    // "\n"     => Token::Eof,
};

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnterminatedString { quote : char, pos : usize },
    InvalidNumber { raw: String, pos: usize },
    UnexpectedChar { ch: char, pos: usize },
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnterminatedString { quote, pos } =>
                write!(f, "Unterminated string literal (opened with `{quote}` at position {pos})"),
            Self::InvalidNumber { raw, pos } =>
                write!(f, "Invalid numeric literal `{raw}` at position {pos}"),
            Self::UnexpectedChar { ch, pos } =>
                write!(f, "Unexpected character `{ch}` at position {pos}"),
        }
    }
}

impl std::error::Error for TokenizeError {}

pub struct Tokenizer{
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer{
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn read_while(&mut self, func: impl Fn(char) -> bool) -> String {
        let mut s = "".to_string();
        while self.peek().map_or(false, |ch| func(ch)) {
            s.push(self.advance().unwrap());
        }
        s
    }

    pub fn next_token(&mut self) -> Option<Result<Token, TokenizeError>> {
        self.read_while(|ch| ch.is_whitespace());
        let pos = self.pos;

        let ch = self.peek()?;
        let token = match ch {
            // Keyword or identifier
            'A'..='Z' | 'a'..='z' | '_'  => {
                let word = self.read_while(|ch| ch.is_alphanumeric() || ch == '_');
                Ok(KEYWORDS
                    .get(&(word.to_uppercase()))
                    .cloned()
                    .unwrap_or(Token::Identifier(word.to_uppercase())))
            }
            // String Literal
            '\"' | '\'' => {
                let quote = self.advance().unwrap();
                let s = self.read_while(|ch| ch != quote);
                match self.advance() {
                    Some(_) => Ok(Token::StringLiteral(s)),
                    None    => Err(TokenizeError::UnterminatedString{ quote: quote, pos: pos })
                }
            }
            // Integer or Float 
            '0'..='9' | '.' => {
                let num = self.read_while(|ch| ch.is_ascii_digit() || ch == '.');
                if num.contains('.') { 
                    match num.parse::<f64>() {
                        Ok(f) => Ok(Token::Float(f)),
                        Err(_) => Err(TokenizeError::InvalidNumber{ raw: num, pos: pos })
                    }
                } else {
                    match num.parse::<i64>() {
                        Ok(i) => Ok(Token::Integer(i)),
                        Err(_) => Err(TokenizeError::InvalidNumber{ raw: num, pos: pos })
                    }
                }
            }
            '=' => { self.advance(); if self.peek() == Some('=') { self.advance(); Ok(Token::EqEq) } else { Ok(Token::Eq) } }
            '<' => { self.advance(); if self.peek() == Some('=') { self.advance(); Ok(Token::LtEq) } else { Ok(Token::Lt) } }
            '>' => { self.advance(); if self.peek() == Some('=') { self.advance(); Ok(Token::GtEq) } else { Ok(Token::Gt) } }
            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Ok(Token::NotEq)
                } else {
                    Err(TokenizeError::UnexpectedChar { ch: '!', pos: self.pos - 1 })
                }
            }
            ',' => { self.advance(); Ok(Token::Comma    ) }
            // '.' => { self.advance(); Ok(Token::Dot      ) }
            '(' => { self.advance(); Ok(Token::LParen   ) }
            ')' => { self.advance(); Ok(Token::RParen   ) }
            ';' => { self.advance(); Ok(Token::Semicolon) }
            '*' => { self.advance(); Ok(Token::Asterisk ) }
            _   => { Err(TokenizeError::UnexpectedChar{ ch: self.chars[pos], pos: pos }) }
        };

        Some(token)
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
        Self::new(input).collect()
    }
}

impl Iterator for Tokenizer {
    type Item = Result<Token, TokenizeError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
