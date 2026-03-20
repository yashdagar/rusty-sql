mod sql;

#[cfg(test)]
mod tests {
    use super::*;

    use sql::token::Token;
    use sql::tokenizer::{Tokenizer, TokenizeError};

    // ── helpers ──────────────────────────────────────────────────────────────

    fn tok(input: &str) -> Result<Vec<Token>, TokenizeError> {
        Tokenizer::tokenize(input)
    }

    fn ok(input: &str) -> Vec<Token> {
        tok(input).expect("expected successful tokenization")
    }

    // ── keywords ─────────────────────────────────────────────────────────────

    #[test]
    fn test_keywords_lowercase() {
        assert_eq!(ok("select"), vec![Token::Select]);
        assert_eq!(ok("from"),   vec![Token::From]);
        assert_eq!(ok("where"),  vec![Token::Where]);
    }

    #[test]
    fn test_keywords_uppercase() {
        assert_eq!(ok("SELECT"), vec![Token::Select]);
        assert_eq!(ok("FROM"),   vec![Token::From]);
        assert_eq!(ok("WHERE"),  vec![Token::Where]);
    }

    #[test]
    fn test_keywords_mixed_case() {
        assert_eq!(ok("Select"), vec![Token::Select]);
        assert_eq!(ok("FrOm"),   vec![Token::From]);
        assert_eq!(ok("WhErE"),  vec![Token::Where]);
    }

    #[test]
    fn test_all_keywords() {
        let cases = vec![
            ("INSERT", Token::Insert),
            ("INTO",   Token::Into),
            ("VALUES", Token::Values),
            ("UPDATE", Token::Update),
            ("SET",    Token::Set),
            ("DELETE", Token::Delete),
            ("CREATE", Token::Create),
            ("TABLE",  Token::Table),
            ("DROP",   Token::Drop),
            ("AND",    Token::And),
            ("OR",     Token::Or),
            ("NOT",    Token::Not),
            ("NULL",   Token::Null),
            ("IS",     Token::Is),
        ];
        for (input, expected) in cases {
            assert_eq!(ok(input), vec![expected], "failed on keyword: {input}");
        }
    }

    // ── identifiers ──────────────────────────────────────────────────────────

    #[test]
    fn test_identifier_simple() {
        assert_eq!(ok("foo"), vec![Token::Identifier("FOO".into())]);
    }

    #[test]
    fn test_identifier_with_underscore() {
        assert_eq!(ok("my_table"), vec![Token::Identifier("MY_TABLE".into())]);
    }

    #[test]
    fn test_identifier_leading_underscore() {
        assert_eq!(ok("_col"), vec![Token::Identifier("_COL".into())]);
    }

    #[test]
    fn test_identifier_with_digits() {
        assert_eq!(ok("col1"), vec![Token::Identifier("COL1".into())]);
    }

    #[test]
    fn test_identifier_is_uppercased() {
        assert_eq!(ok("myCol"), vec![Token::Identifier("MYCOL".into())]);
    }

    // ── string literals ───────────────────────────────────────────────────────

    #[test]
    fn test_single_quoted_string() {
        assert_eq!(ok("'hello'"), vec![Token::StringLiteral("hello".into())]);
    }

    #[test]
    fn test_double_quoted_string() {
        assert_eq!(ok("\"world\""), vec![Token::StringLiteral("world".into())]);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(ok("''"), vec![Token::StringLiteral("".into())]);
    }

    #[test]
    fn test_string_with_spaces() {
        assert_eq!(ok("'hello world'"), vec![Token::StringLiteral("hello world".into())]);
    }

    #[test]
    fn test_unterminated_single_quote() {
        assert_eq!(
            tok("'hello"),
            Err(TokenizeError::UnterminatedString { quote: '\'', pos: 0 })
        );
    }

    #[test]
    fn test_unterminated_double_quote() {
        assert_eq!(
            tok("\"hello"),
            Err(TokenizeError::UnterminatedString { quote: '"', pos: 0 })
        );
    }

    // ── numeric literals ──────────────────────────────────────────────────────

    #[test]
    fn test_integer() {
        assert_eq!(ok("42"), vec![Token::Integer(42)]);
    }

    #[test]
    fn test_integer_zero() {
        assert_eq!(ok("0"), vec![Token::Integer(0)]);
    }

    #[test]
    fn test_float() {
        assert_eq!(ok("3.14"), vec![Token::Float(3.14)]);
    }

    #[test]
    fn test_float_leading_dot() {
        assert_eq!(ok(".5"), vec![Token::Float(0.5)]);
    }

    #[test]
    fn test_invalid_float_multiple_dots() {
        assert!(matches!(
            tok("1.2.3"),
            Err(TokenizeError::InvalidNumber { .. })
        ));
    }

    // ── operators ─────────────────────────────────────────────────────────────

    #[test]
    fn test_eq() {
        assert_eq!(ok("="), vec![Token::Eq]);
    }

    #[test]
    fn test_eqeq() {
        assert_eq!(ok("=="), vec![Token::EqEq]);
    }

    #[test]
    fn test_lt() {
        assert_eq!(ok("<"), vec![Token::Lt]);
    }

    #[test]
    fn test_lt_eq() {
        assert_eq!(ok("<="), vec![Token::LtEq]);
    }

    #[test]
    fn test_gt() {
        assert_eq!(ok(">"), vec![Token::Gt]);
    }

    #[test]
    fn test_gt_eq() {
        assert_eq!(ok(">="), vec![Token::GtEq]);
    }

    #[test]
    fn test_not_eq() {
        assert_eq!(ok("!="), vec![Token::NotEq]);
    }

    #[test]
    fn test_bare_exclamation_is_error() {
        assert_eq!(
            tok("!"),
            Err(TokenizeError::UnexpectedChar { ch: '!', pos: 0 })
        );
    }

    // ── punctuation ───────────────────────────────────────────────────────────

    #[test]
    fn test_punctuation() {
        assert_eq!(ok(","), vec![Token::Comma]);
        assert_eq!(ok("("), vec![Token::LParen]);
        assert_eq!(ok(")"), vec![Token::RParen]);
        assert_eq!(ok(";"), vec![Token::Semicolon]);
        assert_eq!(ok("*"), vec![Token::Asterisk]);
    }

    // ── whitespace handling ───────────────────────────────────────────────────

    #[test]
    fn test_whitespace_skipped() {
        assert_eq!(ok("  SELECT  "), vec![Token::Select]);
    }

    #[test]
    fn test_tabs_and_newlines_skipped() {
        assert_eq!(ok("\t\nSELECT\n"), vec![Token::Select]);
    }

    // ── unexpected characters ─────────────────────────────────────────────────

    #[test]
    fn test_unexpected_char() {
        assert_eq!(
            tok("@"),
            Err(TokenizeError::UnexpectedChar { ch: '@', pos: 0 })
        );
    }

    #[test]
    fn test_unexpected_char_position() {
        // "SELECT @" — the '@' is at position 7
        assert_eq!(
            tok("SELECT @"),
            Err(TokenizeError::UnexpectedChar { ch: '@', pos: 7 })
        );
    }

    // ── empty input ───────────────────────────────────────────────────────────

    #[test]
    fn test_empty_input() {
        assert_eq!(ok(""), vec![]);
    }

    #[test]
    fn test_only_whitespace() {
        assert_eq!(ok("   \t\n  "), vec![]);
    }

    // ── multi-token sequences ─────────────────────────────────────────────────

    #[test]
    fn test_select_star_from() {
        assert_eq!(
            ok("SELECT * FROM users"),
            vec![Token::Select, Token::Asterisk, Token::From, Token::Identifier("USERS".into())]
        );
    }

    #[test]
    fn test_select_columns() {
        assert_eq!(
            ok("SELECT id, name FROM users"),
            vec![
                Token::Select,
                Token::Identifier("ID".into()),
                Token::Comma,
                Token::Identifier("NAME".into()),
                Token::From,
                Token::Identifier("USERS".into()),
            ]
        );
    }

    #[test]
    fn test_where_clause() {
        assert_eq!(
            ok("WHERE age > 18"),
            vec![
                Token::Where,
                Token::Identifier("AGE".into()),
                Token::Gt,
                Token::Integer(18),
            ]
        );
    }

    #[test]
    fn test_insert_statement() {
        assert_eq!(
            ok("INSERT INTO users VALUES (1, 'Alice')"),
            vec![
                Token::Insert,
                Token::Into,
                Token::Identifier("USERS".into()),
                Token::Values,
                Token::LParen,
                Token::Integer(1),
                Token::Comma,
                Token::StringLiteral("Alice".into()),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_update_statement() {
        assert_eq!(
            ok("UPDATE users SET name = 'Bob' WHERE id == 1"),
            vec![
                Token::Update,
                Token::Identifier("USERS".into()),
                Token::Set,
                Token::Identifier("NAME".into()),
                Token::Eq,
                Token::StringLiteral("Bob".into()),
                Token::Where,
                Token::Identifier("ID".into()),
                Token::EqEq,
                Token::Integer(1),
            ]
        );
    }

    #[test]
    fn test_create_table() {
        assert_eq!(
            ok("CREATE TABLE foo (id, name);"),
            vec![
                Token::Create,
                Token::Table,
                Token::Identifier("FOO".into()),
                Token::LParen,
                Token::Identifier("ID".into()),
                Token::Comma,
                Token::Identifier("NAME".into()),
                Token::RParen,
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_null_and_is() {
        assert_eq!(
            ok("WHERE col IS NULL"),
            vec![
                Token::Where,
                Token::Identifier("COL".into()),
                Token::Is,
                Token::Null,
            ]
        );
    }

    #[test]
    fn test_and_or_not() {
        assert_eq!(
            ok("a AND b OR NOT c"),
            vec![
                Token::Identifier("A".into()),
                Token::And,
                Token::Identifier("B".into()),
                Token::Or,
                Token::Not,
                Token::Identifier("C".into()),
            ]
        );
    }

    #[test]
    fn test_drop_table() {
        assert_eq!(
            ok("DROP TABLE users;"),
            vec![
                Token::Drop,
                Token::Table,
                Token::Identifier("USERS".into()),
                Token::Semicolon,
            ]
        );
    }

    // ── iterator interface ────────────────────────────────────────────────────

    #[test]
    fn test_iterator_stops_after_error() {
        let mut t = Tokenizer::new("SELECT @ FROM");
        assert_eq!(t.next(), Some(Ok(Token::Select)));
        assert!(matches!(t.next(), Some(Err(TokenizeError::UnexpectedChar { ch: '@', .. }))));
        // collect() on the iterator stops at first Err, so behaviour is consistent
    }

    #[test]
    fn test_tokenize_fn_is_alias_for_collect() {
        let via_fn  = Tokenizer::tokenize("SELECT *");
        let via_new: Result<Vec<_>, _> = Tokenizer::new("SELECT *").collect();
        assert_eq!(via_fn, via_new);
    }

    // ── Display for TokenizeError ─────────────────────────────────────────────

    #[test]
    fn test_error_display_unterminated_string() {
        let e = TokenizeError::UnterminatedString { quote: '\'', pos: 3 };
        assert!(e.to_string().contains('\''));
        assert!(e.to_string().contains('3'));
    }

    #[test]
    fn test_error_display_invalid_number() {
        let e = TokenizeError::InvalidNumber { raw: "1.2.3".into(), pos: 0 };
        assert!(e.to_string().contains("1.2.3"));
    }

    #[test]
    fn test_error_display_unexpected_char() {
        let e = TokenizeError::UnexpectedChar { ch: '@', pos: 5 };
        assert!(e.to_string().contains('@'));
        assert!(e.to_string().contains('5'));
    }
}