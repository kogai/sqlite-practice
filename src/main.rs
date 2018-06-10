use std::io::{self, Write};
use std::process::exit;

pub fn run() {
    let prompt = "sqlite> ";
    let mut input_buffer = String::new();

    loop {
        io::stdout().write_all(&mut prompt.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input_buffer).unwrap();
        let cmd = input_buffer.to_owned();
        match cmd.as_str().trim() {
            ".exit" => exit(0),
            x => {
                println!(r#"Unrecoganized command "{}"."#, x);
                input_buffer = "".to_string();
            }
        };
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
