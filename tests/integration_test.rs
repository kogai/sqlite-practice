use std::env::current_dir;
use std::io::Write;
use std::process::{Command, Stdio};

fn setup_each() {
  let cmd = Command::new("make").arg("clean").output().unwrap();
  println!("{}", String::from_utf8_lossy(cmd.stdout.as_slice()));
}

macro_rules! assert_command {
  ($cmd:expr, $expect:expr) => {
    let mut cwd = current_dir().expect("To get current directory failed.");
    cwd.push("target/debug/sqlite");
    let mut child = Command::new(cwd)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("Failed to execute command");

    child
      .stdin
      .as_mut()
      .expect("Failed to open stdin")
      .write_all(
        $cmd
          .into_iter()
          .flat_map(|s| format!("{}\n", s).as_bytes().to_owned())
          .collect::<Vec<_>>()
          .as_slice(),
      )
      .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    let actual =
      String::from_utf8(output.stdout.into_iter().filter(|b| b > &0).collect()).unwrap();
    let expect = format!(
      r#"SQLite version 0.1.0 2018-06-04 19:24:41
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
{}sqlite> "#,
      $expect
        .into_iter()
        .map(|s| format!("sqlite> {}\n", s))
        .collect::<String>()
    );
    assert_eq!(actual, expect);
  };
}

#[test]
fn e2e_insert() {
  setup_each();
  assert_command!(
    vec!["insert 1 foo 'a@b.c';", "select;"],
    vec![
      "Insert successed.",
      "[Row { id: 1, username: foo, email: a@b.c }]",
    ]
  );
}

#[test]
fn e2e_persist() {
  setup_each();
  assert_command!(
    vec!["insert 1 foo 'a@b.c';", "select;"],
    vec![
      "Insert successed.",
      "[Row { id: 1, username: foo, email: a@b.c }]",
    ]
  );
  assert_command!(
    vec!["select;"],
    vec!["[Row { id: 1, username: foo, email: a@b.c }]"]
  );
}

#[test]
fn e2e_seek() {
  use std::fs::File;
  use std::io::{Read, Seek, SeekFrom};
  let mut file = File::open("./Cargo.toml").unwrap();
  let _ = file.seek(SeekFrom::Start(10));
  let mut buf = [0; 4];
  let _ = file.read_exact(&mut buf);
  assert_eq!("name".to_owned(), String::from_utf8_lossy(&buf));
}
