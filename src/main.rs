mod ast;
mod pager;
mod row;
mod table;
mod token;

use ast::{Ast, Parser};
use row::{Definition, Row};
use std::io::{self, Write};
use std::process::exit;
use table::Table;
use token::Lexer;

fn parse(raw_query: String, tbl: &mut Table) -> Vec<u8> {
    let expressions = Parser::new(Lexer::new(raw_query)).parse();
    match expressions.get(0).unwrap() {
        Ast::InsertExpression(statements) => {
            let id = u32::from_str_radix(statements.get(0).unwrap().as_ref(), 10).unwrap();
            let username = statements.get(1).unwrap().to_owned();
            let email = statements.get(2).unwrap().to_owned();
            let row = Row::ser(id, username, email, &tbl.def);
            tbl.insert(row);
            b"Insert successed.\n".to_vec()
        }
        Ast::SelectExpression => {
            let result = tbl.select();
            format!("{:?}\n", result).as_bytes().to_vec()
        }
        Ast::DeleteExpression => b"Not implemented yet.\n".to_vec(),
    }
}

fn run() {
    let prompt = "sqlite> ";
    let mut input_buffer = String::new();
    let def = Definition::new();
    let mut tbl = Table::open_db(None, &def);

    loop {
        io::stdout().write_all(&mut prompt.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        // Handle a case for input only EOF.
        if let Ok(0) = io::stdin().read_line(&mut input_buffer) {
            exit(0);
        }
        let cmd = input_buffer.to_owned();
        let mut result = match cmd.as_str().trim() {
            ".exit" => exit(0),
            "" => vec![],
            cmd => parse(cmd.to_owned(), &mut tbl),
        };
        io::stdout().write_all(&mut result).unwrap();
        io::stdout().flush().unwrap();
        input_buffer = "".to_string();
    }
}

fn main() {
    io::stdout()
        .write_all(&mut r#"SQLite version 0.1.0 2018-06-04 19:24:41
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
"#.as_bytes())
        .unwrap();
    run();
}
