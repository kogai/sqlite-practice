use std::io::Write;
use std::process::{Command, Stdio};

// TODO: Investigate whether can eliminate unintended input of newline code.
macro_rules! assert_command {
  ($cmd:expr, $expect:expr) => {
    let mut child = Command::new("cargo")
      .arg("run")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("Failed to execute command");

    child
      .stdin
      .as_mut()
      .expect("Failed to open stdin")
      .write_all($cmd.as_bytes())
      .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert_eq!(
      String::from_utf8_lossy(&output.stdout),
      format!(
        r#"SQLite version 0.1.0 2018-06-04 19:24:41
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite> {}
sqlite> "#,
        $expect
      )
    );
  };
}

#[test]
fn e2e_insert() {
  assert_command!("insert 1 foo 'a@b.c'; select;", "Insert successed.");
}

#[test]
fn e2e_select() {
  assert_command!("select;", "[]");
}
