mod ast;
mod token;

use ast::Parser;
use std::io::{self, Write};
use std::process::exit;
use token::Lexer;

/*
id: integer 4bytes
username: varchar(32) 32bytes
email: varchar(255) 255bytes
NOTE: For simplicity, we hardcode a user schema as above.
*/

fn parse(raw_query: String) -> Vec<u8> {
    let expressions = Parser::new(Lexer::new(raw_query)).parse();
    println!("{:?}", expressions);
    b"Command has not implemented.\n".to_vec()
}

fn run() {
    let prompt = "sqlite> ";
    let mut input_buffer = String::new();

    loop {
        io::stdout().write_all(&mut prompt.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input_buffer).unwrap();
        let cmd = input_buffer.to_owned();
        let mut result = match cmd.as_str().trim() {
            ".exit" => exit(0),
            cmd => parse(cmd.to_owned()),
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
