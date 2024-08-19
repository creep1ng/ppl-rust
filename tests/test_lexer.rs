extern crate ppl_rust;

#[cfg(test)]
mod test_lexer {

    use ppl_rust::lexer::*;
    use ppl_rust::token::*;

    #[test]
    fn test_illegal() {
        let source: &str = "¡¿@";
        let mut lexer: Lexer = Lexer::new(source);

        let mut tokens: Vec<Token> = Vec::new();
        for _ in source.chars() {
            tokens.push(lexer.next_token())
        }

        let expected_tokens: Vec<Token> = vec![
            Token {
                token_type: TokenType::Illegal,
                literal: Some('¡'),
            },
            Token {
                token_type: TokenType::Illegal,
                literal: Some('¿'),
            },
            Token {
                token_type: TokenType::Illegal,
                literal: Some('@'),
            },
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_one_character_operator() {
        let source: &str = "=+";
        let mut lexer: Lexer = Lexer::new(source);

        let mut tokens: Vec<Token> = Vec::new();
        for _ in source.chars() {
            tokens.push(lexer.next_token())
        }

        let expected_tokens: Vec<Token> = vec![
            Token {
                token_type: TokenType::Assign,
                literal: Some('='),
            },
            Token {
                token_type: TokenType::Plus,
                literal: Some('+'),
            },
        ];

        assert_eq!(tokens, expected_tokens)
    }

    #[test]
    fn test_eof() {
        let source: &str = "+";
        let mut lexer = Lexer::new(source);

        let mut tokens: Vec<Token> = Vec::new();
        for _ in 0..source.len() + 1 {
            tokens.push(lexer.next_token())
        }

        let expected_tokens: Vec<Token> = vec![
            Token {
                token_type: TokenType::Plus,
                literal: Some('+'),
            },
            Token {
                token_type: TokenType::EOF,
                literal: None,
            },
        ];

        assert_eq!(tokens, expected_tokens)
    }

    #[test]
    fn test_delimiters() {
        let source: &str = "(){},;";
        let mut lexer = Lexer::new(source);

        let mut tokens: Vec<Token> = Vec::new();
        for _ in source.chars() {
            tokens.push(lexer.next_token())
        }

        let expected_tokens: Vec<Token> = vec![
            Token {
                token_type: TokenType::LParen,
                literal: Some('('),
            },
            Token {
                token_type: TokenType::RParen,
                literal: Some(')'),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: Some('{'),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: Some('}'),
            },
            Token {
                token_type: TokenType::Comma,
                literal: Some(','),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: Some(';'),
            },
        ];

        assert_eq!(tokens, expected_tokens)
    }
}
