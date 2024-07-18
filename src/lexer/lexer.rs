#![allow(dead_code)]

use crate::token::token;
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

  pub fn next_token(&mut self) -> token::Token {
    let tok: token::Token;

    self.skip_whitespace();

    match self.curr_char {
      Some(',') => tok = new_token(token::TokenType::COMMA, ",".to_string()),
      Some(';') => tok = new_token(token::TokenType::SEMICOLON, ";".to_string()),

      // Bracket and paren
      Some('(') => tok = new_token(token::TokenType::LPAREN, "(".to_string()),
      Some(')') => tok = new_token(token::TokenType::RPAREN, ")".to_string()),
      Some('{') => tok = new_token(token::TokenType::LBRACE, "{".to_string()),
      Some('}') => tok = new_token(token::TokenType::RBRACE, "}".to_string()),

      // Operators
      Some('+') => tok = new_token(token::TokenType::PLUS, "+".to_string()),

      // Possible Double char operators
      Some('!') => {
        if self.peek_next_char() == Some('=') {
          self.read_char();
          tok = new_token(token::TokenType::NOTEQ, "!=".to_string())
        } else {
          tok = new_token(token::TokenType::BANG, "!".to_string())
        }
      }
      Some('=') => {
        if self.peek_next_char() == Some('=') {
          self.read_char();
          tok = new_token(token::TokenType::EQ, "==".to_string())
        } else {
          tok = new_token(token::TokenType::ASSIGN, "=".to_string())
        }
      }

      // Ident and keywords
      // or illegal
      Some(c) => {
        if is_letter(c) {
          let ident = self.read_ident();

          return token::Token {
            literal: ident.clone(),
            token_type: token::lookup_ident(&ident),
          };
        }

        if is_digit(c) {
          return token::Token {
            literal: self.read_number(),
            token_type: token::TokenType::INT,
          };
        }

        tok = new_token(token::TokenType::ILLEGAL, c.to_string());
      }

      // EOF
      None => {
        tok = new_token(token::TokenType::EOF, "".to_string());
      }
    }

    self.read_char();

    tok
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

fn new_token(tt: token::TokenType, literal: String) -> token::Token {
  token::Token {
    token_type: tt,
    literal,
  }
}
