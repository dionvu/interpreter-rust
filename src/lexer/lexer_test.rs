#![allow(dead_code)]

use crate::token::token::TokenType;

struct LexerTest {
  expected_type: TokenType,
  expected_literal: String,
}

#[cfg(test)]
mod tests {
  use crate::lexer::lexer;
  use crate::lexer::lexer_test::LexerTest;
  use crate::token::token::TokenType;

  #[test]
  fn lexer_test() {
    let input = "               
      let five = 5;
      let add = fn(x, y) {
        x + y;
      };

      != == while!
    "
    .to_string();

    let mut lexer = lexer::Lexer::new(input);

    let tests: Vec<LexerTest> = vec![
      LexerTest {
        expected_type: TokenType::LET,
        expected_literal: "let".to_string(),
      },
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "five".to_string(),
      },
      LexerTest {
        expected_type: TokenType::ASSIGN,
        expected_literal: "=".to_string(),
      },
      LexerTest {
        expected_type: TokenType::INT,
        expected_literal: "5".to_string(),
      },
      LexerTest {
        expected_type: TokenType::SEMICOLON,
        expected_literal: ";".to_string(),
      },
      // NEW LINE
      // NEW LINE
      // NEW LINE
      LexerTest {
        expected_type: TokenType::LET,
        expected_literal: "let".to_string(),
      },
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "add".to_string(),
      },
      LexerTest {
        expected_type: TokenType::ASSIGN,
        expected_literal: "=".to_string(),
      },
      LexerTest {
        expected_type: TokenType::FUNCTION,
        expected_literal: "fn".to_string(),
      },
      LexerTest {
        expected_type: TokenType::LPAREN,
        expected_literal: "(".to_string(),
      },
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "x".to_string(),
      },
      LexerTest {
        expected_type: TokenType::COMMA,
        expected_literal: ",".to_string(),
      },
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "y".to_string(),
      },
      LexerTest {
        expected_type: TokenType::RPAREN,
        expected_literal: ")".to_string(),
      },
      LexerTest {
        expected_type: TokenType::LBRACE,
        expected_literal: "{".to_string(),
      },
      // NEW LINE
      // NEW LINE
      // NEW LINE
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "x".to_string(),
      },
      LexerTest {
        expected_type: TokenType::PLUS,
        expected_literal: "+".to_string(),
      },
      LexerTest {
        expected_type: TokenType::IDENT,
        expected_literal: "y".to_string(),
      },
      LexerTest {
        expected_type: TokenType::SEMICOLON,
        expected_literal: ";".to_string(),
      },
      // NEW LINE
      // NEW LINE
      // NEW LINE
      LexerTest {
        expected_type: TokenType::RBRACE,
        expected_literal: "}".to_string(),
      },
      LexerTest {
        expected_type: TokenType::SEMICOLON,
        expected_literal: ";".to_string(),
      },
      // NEW LINE
      // NEW LINE
      // NEW LINE
      LexerTest {
        expected_type: TokenType::NOTEQ,
        expected_literal: "!=".to_string(),
      },
      LexerTest {
        expected_type: TokenType::EQ,
        expected_literal: "==".to_string(),
      },
      // NEW LINE
      // NEW LINE
      // NEW LINE
      LexerTest {
        expected_type: TokenType::WHILE,
        expected_literal: "while".to_string(),
      },
      LexerTest {
        expected_type: TokenType::BANG,
        expected_literal: "!".to_string(),
      },
      // EOF
      // EOF
      // EOF
      LexerTest {
        expected_type: TokenType::EOF,
        expected_literal: "".to_string(),
      },
    ];

    for (idx, test) in tests.iter().enumerate() {
      let tok = lexer.next_token();

      println!("Current test: {}", idx);

      assert_eq!(tok.token_type, test.expected_type);
      assert_eq!(tok.literal, test.expected_literal);
    }
  }
}
