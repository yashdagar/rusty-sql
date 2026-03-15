#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,
    Insert,
    Into,
    Values,
    Update,
    Set,
    Delete,
    Create,
    Table,
    Drop,
    And,
    Or,
    Not,
    Null,
    Is,

    // Literals
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    // table names, column names
    Identifier(String),

    // Operators
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Punctuation
    Comma,
    Semicolon,
    LParen,
    RParen,
    Asterisk,
    Dot,

    // Special
    Eof,
}
