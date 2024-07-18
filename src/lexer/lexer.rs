#![allow(dead_code)]

use crate::token::token::{lookup_ident, Token, TokenType};
use crate::utils::utils::{is_digit, is_letter};

pub struct Lexer {
  input: String,
  input_chars: Vec<char>,
  input_len: usize,
  pos: usize,
  next_pos: usize,
  curr_char: Option<char>,
}

impl Lexer {
  pub fn new(input_code: String) -> Self {
    let len = input_code.len();
    let chars: Vec<char> = input_code.chars().collect();

    let mut lexer = Lexer {
      input: input_code,
      input_chars: chars,
      input_len: len,
      pos: 0,
      next_pos: 0,
      curr_char: None,
    };

    lexer.read_char();

    lexer
  }

  fn read_char(&mut self) {
    if self.next_pos >= self.input_len {
      self.curr_char = None;
    } else {
      self.curr_char = Some(self.input_chars[self.next_pos]);
    }

    self.pos = self.next_pos;

    self.next_pos += 1;
  }

  pub fn next_token(&mut self) -> Token {
    let tok: Token;

    self.skip_whitespace();

    match self.curr_char {
      Some(',') => tok = Token::new(TokenType::COMMA, ",".to_string()),
      Some(';') => tok = Token::new(TokenType::SEMICOLON, ";".to_string()),

      // Bracket and paren
      Some('(') => tok = Token::new(TokenType::LPAREN, "(".to_string()),
      Some(')') => tok = Token::new(TokenType::RPAREN, ")".to_string()),
      Some('{') => tok = Token::new(TokenType::LBRACE, "{".to_string()),
      Some('}') => tok = Token::new(TokenType::RBRACE, "}".to_string()),

      // Operators
      Some('+') => tok = Token::new(TokenType::PLUS, "+".to_string()),

      // Possible Double char operators
      Some('!') => {
        if self.peek_next_char() == Some('=') {
          self.read_char();
          tok = Token::new(TokenType::NOTEQ, "!=".to_string())
        } else {
          tok = Token::new(TokenType::BANG, "!".to_string())
        }
      }
      Some('=') => {
        if self.peek_next_char() == Some('=') {
          self.read_char();
          tok = Token::new(TokenType::EQ, "==".to_string())
        } else {
          tok = Token::new(TokenType::ASSIGN, "=".to_string())
        }
      }

      // Ident and keywords
      // or illegal
      Some(c) => {
        if is_letter(c) {
          let ident = self.read_ident();

          return Token {
            literal: ident.clone(),
            token_type: lookup_ident(&ident),
          };
        }

        if is_digit(c) {
          return Token {
            literal: self.read_number(),
            token_type: TokenType::INT,
          };
        }

        tok = Token::new(TokenType::ILLEGAL, c.to_string());
      }

      // EOF
      None => {
        tok = Token::new(TokenType::EOF, "".to_string());
      }
    }

    self.read_char();

    tok
  }

  fn read_ident(&mut self) -> String {
    let starting_pos = self.pos;

    while let Some(c) = self.curr_char {
      if is_letter(c) {
        self.read_char();
      } else {
        break;
      }
    }

    self.input[starting_pos..self.pos].to_string()
  }

  fn read_number(&mut self) -> String {
    let starting_pos = self.pos;

    while let Some(c) = self.curr_char {
      if is_digit(c) {
        self.read_char();
      } else {
        break;
      }
    }

    self.input[starting_pos..self.pos].to_string()
  }

  fn peek_next_char(&self) -> Option<char> {
    if self.next_pos >= self.input_len {
      return None;
    }

    return Some(self.input_chars[self.next_pos]);
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.curr_char {
      match c {
        ' ' => self.read_char(),
        '\n' => self.read_char(),
        _ => break,
      }
    }
  }
}
