use token::{Lexer, Token, TokenType};

#[derive(Debug, PartialEq)]
pub enum Ast {
  InsertExpression(Vec<String>),
  DeleteExpression(Vec<String>),
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
      let ast = self.parse_expression();
      expressions.push(ast);
      self.next_token();
    }
    expressions
  }

  fn parse_expression(&self) -> Ast {
    unimplemented!();
  }

  fn next_token(&mut self) {
    self.current_token = self.peek_token.to_owned();
    self.peek_token = self.lexer.next_token();
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
DELETE foo;"#.to_string(),
      )).parse(),
      vec![
        SelectExpression,
        InsertExpression(vec![]),
        DeleteExpression(vec![]),
      ]
    );
  }
}
