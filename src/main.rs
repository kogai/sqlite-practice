mod ast;
mod token;

use std::io::{self, Write};
use std::process::exit;

fn parse(raw_query: String) -> Vec<u8> {
    unimplemented!();
}
// cmd => println!(r#"Unrecoganized command "{}"."#, cmd),

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
