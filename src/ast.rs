use token::{Lexer, Token};

pub enum Ast {
  InsertExpression(Vec<String>),
  DeleteExpression,
  SelectExpression,
  Yet,
}

#[derive(Debug)]
pub struct Parser {
  lexer: Lexer,
  current_token: Token,
  peek_token: Token,
  // errors: Vec<String>,
}

// impl Parser {
//   pub fn new(mut lexer: Lexer) -> Self {
//     let first = lexer.next_token();
//     let second = lexer.next_token();
//     Parser {
//       lexer: lexer,
//       current_token: first,
//       peek_token: second,
//       errors: vec![],
//     }
//   }

//     pub fn parse_program(&mut self) -> Program {
//         let mut statements: Vec<Statements> = vec![];

//         while self.current_token.token_type != TokenType::EOF {
//             let statement = self.parse_statement();
//             statements.push(statement);
//             self.next_token();
//         }

//         Program { statements: statements }
// }

// }
