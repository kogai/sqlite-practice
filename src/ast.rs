use token::{Lexer, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Ast {
  InsertExpression(Vec<String>),
  DeleteExpression,
  SelectExpression,
}

pub type Expressions = Vec<Ast>;

#[derive(Debug)]
pub struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
  /* TODO
  errors: Vec<String>,
   */
}

impl Parser {
  pub fn new(mut lexer: Lexer) -> Self {
    let current_token = lexer.next_token();
    let peek_token = lexer.next_token();
    Parser {
      lexer,
      current_token,
      peek_token,
    }
  }

  pub fn parse(&mut self) -> Expressions {
    let mut expressions = vec![];
    while self.current_token.token_type != TokenType::Eof {
      expressions.push(self.parse_expression());
      self.next_token();
    }
    expressions
  }

  fn parse_expression(&mut self) -> Ast {
    let ast = match self.current_token.token_type {
      TokenType::Insert => self.parse_insert(),
      TokenType::Delete => {
        self.next_token();
        Ast::DeleteExpression
      }
      TokenType::Select => {
        self.next_token();
        Ast::SelectExpression
      }
      _ => {
        // TODO: Handle error
        unimplemented!();
      }
    };
    ast
  }

  fn next_token(&mut self) {
    self.current_token = self.peek_token.to_owned();
    self.peek_token = self.lexer.next_token();
  }

  fn parse_insert(&mut self) -> Ast {
    self.next_token();

    let mut instructions = vec![];
    while self.current_token.token_type != TokenType::SemiColon {
      let token = self.current_token.literal.to_owned();
      instructions.push(token);
      self.next_token();
    }
    Ast::InsertExpression(instructions)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ast() {
    use self::Ast::*;

    assert_eq!(
      Parser::new(Lexer::new(
        r#"SELECT;
INSERT foo bar;
DELETE;"#.to_string(),
      )).parse(),
      vec![
        SelectExpression,
        InsertExpression(vec!["foo".to_owned(), "bar".to_owned()]),
        DeleteExpression,
      ]
    );
  }

  #[test]
  fn test_parse_insert() {
    use self::Ast::*;

    assert_eq!(
      Parser::new(Lexer::new(r#"INSERT 1 foo bar@buzz.com;"#.to_string(),)).parse(),
      vec![InsertExpression(vec![
        "1".to_owned(),
        "foo".to_owned(),
        "bar@buzz.com".to_owned(),
      ])]
    );
  }
}
