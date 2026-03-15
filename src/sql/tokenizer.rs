use crate::Token;
use phf::phf_map;

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

    // Operators  
    "="      => Token::Eq,
    "!="     => Token::NotEq,
    "<"      => Token::Lt,
    ">"      => Token::Gt,
    "<="     => Token::LtEq,
    ">="     => Token::GtEq,

    // Punctuation
    ","      => Token::Comma,
    "."      => Token::Dot,
    "("      => Token::LParen,
    ")"      => Token::RParen,
    ";"      => Token::Semicolon,
    "*"      => Token::Asterisk,

    // Special
    "\n"     => Token::Eof,
};

pub struct Tokenizer{
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer{
    pub fn tokenize(text: String) -> Vec<Token> {
        let text: Vec<char> = text.to_uppercase().chars().collect();

        let mut i = 0;
        let sz = text.len();

        let mut tokens: Vec<Token> = vec![];
        let mut cur = "".to_string();

        while i < sz {
            if text[i].is_alphabetic() {
                // Identifier or Keyword
                while text[i].is_alphanumeric() {
                    cur.push(text[i]);
                    i += 1;
                }

                // Check if it is a keyword
                if KEYWORDS.contains_key(&cur) {
                    tokens.push(KEYWORDS[&cur].clone());
                }else{
                    tokens.push(Token::Identifier(cur));
                }
                cur = "".to_string();
            } else if text[i] == ',' {
                tokens.push(Token::Comma);
                i += 1;
            } else if text[i] == ';' {
                tokens.push(Token::Semicolon);
                i += 1;
            } else {
                i += 1;
            }
        }

        tokens
    }
}