use core::fmt;

enum TokenType {
    Assign,
    Comma,
    EOF,
    Function,
    Ident,
    Illegal,
    Int,
    LBrace,
    Let,
    LParent,
    Plus,
    RBrace,
    RParen,
    Semicolon,
}

struct Token<'a> {
    token_type: TokenType,
    literal: &'a str,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::Assign => write!(f, "Assign"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Ident => write!(f, "Ident"),
            TokenType::Illegal => write!(f, "Illegal"),
            TokenType::Int => write!(f, "Int"),
            TokenType::LBrace => write!(f, "LBrace"),
            TokenType::Let => write!(f, "Let"),
            TokenType::LParent => write!(f, "LParent"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::RBrace => write!(f, "RBrace"),
            TokenType::RParen => write!(f, "RParen"),
            TokenType::Semicolon => write!(f, "Semicolon"),
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenType: {}, literal: {}",
            self.token_type, self.literal
        )
    }
}
