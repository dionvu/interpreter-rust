#![allow(dead_code)]

#[derive(Clone, Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  // Special characters
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  COMMA,
  SEMICOLON,

  // Operators
  ASSIGN,
  PLUS,
  MINUS,
  BANG,
  ASTERISK,
  SLASH,
  LT,
  GT,

  // 2 char operators
  EQ,
  NOTEQ,

  // Types
  INT,
  IDENT,

  // Keywords
  FUNCTION,
  LET,
  RETURN,
  TRUE,
  FALSE,
  IF,
  ELSE,
  WHILE,
}

/// Differentiates between identifiers and keywords.
///
/// # Examples
/// ```rust
/// assert_eq!(lookup_ident("foo"), TokenType::IDENT);
/// assert_eq!(lookup_ident("return"), TokenType::RETURN);
/// ```
pub fn lookup_ident(ident: &str) -> TokenType {
  match ident {
    "fn" => TokenType::FUNCTION,
    "let" => TokenType::LET,
    "return" => TokenType::RETURN,
    "true" => TokenType::TRUE,
    "false" => TokenType::FALSE,
    "if" => TokenType::IF,
    "else" => TokenType::ELSE,
    "while" => TokenType::WHILE,
    _ => TokenType::IDENT,
  }
}
