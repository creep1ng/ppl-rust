use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a str,
    character: Option<char>,
    read_position: u32,
    position: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            source,
            character: None,
            read_position: 0,
            position: 0,
        };
        lexer.read_character();
        lexer
    }

    /// Returns the next token from the input source.
    pub fn next_token(&mut self) -> Token {
        let token = match self.character {
            Some('+') => Token {
                token_type: TokenType::Plus,
                literal: self.character,
            },
            Some('=') => Token {
                token_type: TokenType::Assign,
                literal: self.character,
            },
            Some(_) => Token {
                token_type: TokenType::Illegal,
                literal: self.character,
            },
            None => Token {
                token_type: TokenType::EOF,
                literal: None,
            },
        };
        self.read_character();
        token
    }

    /// Returns the next token from the input source.
    fn read_character(&mut self) {
        /* Since this function takes ownership over lexer (???), we must pass a mutable
        self reference instead mutable self as the argument of this function. */

        /* We need to make a cast here ──────────────┐
        because source.len() returns an usize, but  │
        read_position is an u32.                   ││*/
        if self.read_position >= self.source.len() as u32 {
            self.character = None;
        } else {
            /* Since we can't slice strings like in Python (because UTF characters are size-variable),
            we must iterate over the str until the `pos`th position of the str. */
            let pos = self.read_position as usize;
            self.character = self.source.chars().nth(pos)
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}
