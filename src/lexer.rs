#[derive(Debug, PartialEq)]
pub enum TokenType {
  Eof,
  Insert,
  Delete,
  Select,
  SemiColon,
  Yet,
}

#[derive(Debug, PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
  pub line: u8,
  pub column: u8,
}

impl Token {
  fn new(literal: String, line: u8, column: u8) -> Self {
    use self::TokenType::*;
    let token_type = match literal.as_str() {
      "INSERT" | "insert" => Insert,
      "SELECT" | "select" => Select,
      "DELETE" | "delete" => Delete,
      ";" => SemiColon,
      "" => Eof,
      _ => Yet,
    };
    Token {
      token_type,
      literal,
      line,
      column,
    }
  }
}

#[derive(Debug, Clone)]
pub struct Lexer {
  input: Vec<char>,
  current_char: char,
  position: u32,
  read_position: u32,
  line: u8,
  column: u8,
}

impl Lexer {
  pub fn new(input: String) -> Self {
    let input: Vec<char> = input.chars().collect();
    let current_char = input.get(0).expect("Empty string!").to_owned();

    let mut lexer = Lexer {
      input,
      current_char,
      position: 1,
      read_position: 2,
      line: 1,
      column: 1,
    };
    lexer
  }

  fn skip_white_space(&mut self) {
    match self.current_char {
      '\n' | '\r' => {
        self.column = 0;
        self.line = self.line + 1;
      }
      ' ' | '　' | '\t' => {
        self.read_char();
        self.skip_white_space();
      }
      _ => {}
    };
  }

  fn nth_char(&self, position: u32) -> char {
    self
      .input
      .get(self.position as usize)
      .expect(&format!("Index of input {} is out of range", self.position))
      .to_owned()
  }

  fn peek_char(&self) -> char {
    self.nth_char(self.read_position - 1)
  }

  fn read_char(&mut self) {
    self.current_char = self.nth_char(self.position);
    self.position = self.read_position;
    self.column += 1;
    self.read_position += 1;
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_white_space();
    let current_char = self.current_char;
    let column = self.column;
    let line = self.line;

    // let seed = match current_char {
    //   x if is_letter(x) => self.read_identifier(),
    //   x if is_digit(x) => self.read_digit(),
    //   x if x == "=" && self.peak_char() == "=" => {
    //     self.read_char();
    //     self.read_char();
    //     format!("{}{}", x, "=")
    //   }
    //   x if x == "!" && self.peak_char() == "=" => {
    //     self.read_char();
    //     self.read_char();
    //     format!("{}{}", x, "=")
    //   }
    //   x if x == "\"" => {
    //     is_string = true;
    //     self.read_string()
    //   }
    //   x => {
    //     self.read_char();
    //     x.clone()
    //   }
    // };
    // token::Token::new(seed, is_string, self.line, position)
    unimplemented!();
  }

  fn read_identifier(&mut self) -> String {
    // let start = (self.position - 1) as usize;

    // while is_letter(&self.current_char) {
    //   self.read_char();
    // }

    // let input_chars = self.input.chars().collect::<Vec<char>>();
    // let end = (self.position - 1) as usize;
    // let splited = &input_chars[start..end]
    //   .iter()
    //   .fold("".to_string(), |acc, &s| {
    //     format!("{}{}", acc, s.to_string())
    //   });
    // (*splited).to_string()
    unimplemented!();
  }

  /*
  fn read_string(&mut self) -> String {
    self.read_char();
    let start = (self.position - 1) as usize;

    while self.current_char != "\"" {
      self.read_char();
    }

    let input_chars = self.input.chars().collect::<Vec<char>>();
    let end = (self.position - 1) as usize;
    self.read_char();

    (&input_chars[start..end])
      .iter()
      .fold("".to_string(), |acc, &s| {
        format!("{}{}", acc, s.to_string())
      })
  }
  fn read_digit(&mut self) -> String {
    let start = (self.position - 1) as usize;

    while is_digit(&self.current_char) {
      self.read_char();
    }

    let input_chars = self.input.chars().collect::<Vec<char>>();
    let end = (self.position - 1) as usize;
    let splited = &input_chars[start..end]
      .iter()
      .fold("".to_string(), |acc, &s| {
        format!("{}{}", acc, s.to_string())
      });
    (*splited).to_string()
  }

  */
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tokenize() {
    let mut l = Lexer::new(
      r#"SELECT;
INSERT foo bar;
DELETE foo;"#.to_string(),
    );
    use self::TokenType::*;
    let expects = vec![
      (Select, "SELECT"),
      (SemiColon, ";"),
      (Insert, "INSERT"),
      (Yet, "foo"),
      (Yet, "bar"),
      (SemiColon, ";"),
      (Delete, "Delete"),
      (Yet, "foo"),
      (SemiColon, ";"),
      (Eof, ""),
    ];

    for (token_type, literal) in expects {
      let t = l.next_token();
      assert_eq!(t.token_type, token_type);
      assert_eq!(t.literal, literal);
    }
  }
}
