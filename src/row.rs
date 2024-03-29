use std::collections::HashMap;
use std::fmt;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
enum DataType {
  TEXT(usize),
  INTEGER,
  // NUMERIC,
  // REAL,
  // NONE,
}

impl DataType {
  fn size(&self) -> usize {
    use self::DataType::*;
    match self {
      &TEXT(size) => size,
      &INTEGER => 10,
    }
  }
}

#[derive(Debug)]
pub struct Definition {
  definitions: HashMap<String, DataType>,
  pub row_size: usize,
  pub row_per_page: usize,
}

impl Definition {
  pub fn new() -> Self {
    use self::DataType::*;

    // Hardcoded definition of the row.
    let mut hs = HashMap::new();
    hs.insert("id".to_owned(), INTEGER);
    hs.insert("username".to_owned(), TEXT(32));
    hs.insert("email".to_owned(), TEXT(255));
    let row_size = hs.iter().fold(0, |acc, (_, v)| acc + v.size());
    let row_per_page = PAGE_SIZE / row_size;

    Definition {
      definitions: hs,
      row_size,
      row_per_page,
    }
  }

  fn size_of(&self, key: &str) -> usize {
    let value = self
      .definitions
      .get(key)
      .expect("Wrong key has been passed");
    value.size()
  }
}

pub struct Row {
  pub data: Vec<u8>,
  id: u32,
  username: String,
  email: String,
}

impl Row {
  pub fn truncate_tail(source: Vec<u8>) -> Vec<u8> {
    source
      .iter()
      .rev()
      .skip_while(|x| **x == 0)
      .collect::<Vec<_>>()
      .iter()
      .rev()
      .map(|x| **x)
      .collect()
  }

  pub fn ser(id: u32, username: String, email: String, def: &Definition) -> Self {
    let mut buf_id = format!("{}", id).as_bytes().to_vec();
    buf_id.resize(def.size_of("id"), 0);

    let mut bu_username = username.as_bytes().to_vec();
    bu_username.resize(def.size_of("username"), 0);

    let mut buf_email = email.as_bytes().to_vec();
    buf_email.resize(def.size_of("email"), 0);

    let serialized = [&buf_id[..], &bu_username[..], &buf_email].concat();
    if serialized.len() != def.row_size {
      panic!("Wrong size of row");
    }

    Row {
      data: serialized,
      id,
      username,
      email,
    }
  }

  pub fn de(source: &Vec<u8>, def: &Definition) -> Result<Self, Error> {
    use std::str::from_utf8;

    let id_offset = 0;
    let username_offset = id_offset + def.size_of("id");
    let email_offset = username_offset + def.size_of("username");

    let id = &source[0..username_offset];
    let username = &source[username_offset..email_offset];
    let email = &source[email_offset..];

    match (id.get(0), username.get(0), email.get(0)) {
      (Some(x), Some(y), Some(z)) if *x > 0 && *y > 0 && *z > 0 => {
        let mut id = Row::truncate_tail(id.to_vec());
        let mut username = Row::truncate_tail(username.to_vec());
        let mut email = Row::truncate_tail(email.to_vec());

        let id = u32::from_str_radix(from_utf8(id.as_slice()).unwrap(), 10).unwrap();
        let username = from_utf8(username.as_slice()).unwrap().to_owned();
        let email = from_utf8(email.as_slice()).unwrap().to_owned();
        Ok(Row {
          data: source.to_owned(),
          id,
          username,
          email,
        })
      }
      _ => Err(Error),
    }
  }
}

impl fmt::Debug for Row {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Row {{ id: {}, username: {}, email: {} }}",
      self.id, self.username, self.email
    )
  }
}

impl PartialEq for Row {
  fn eq(&self, other: &Row) -> bool {
    self.data == other.data
  }
}
