use pager::Pager;
use std::fmt;
use std::mem::transmute;

/*
id: integer 4bytes
username: varchar(32) 32bytes
email: varchar(255) 255bytes
NOTE: For simplicity, we hardcode a user schema as above.
*/
const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
pub const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

pub struct Row {
  id: u32,
  username: [u8; USERNAME_SIZE],
  email: [u8; EMAIL_SIZE],
}

impl Row {
  pub fn new(id: u32, username: String, email: String) -> Self {
    let mut buf_username = [0; USERNAME_SIZE];
    let mut buf_email = [0; EMAIL_SIZE];
    let mut rest_username = vec![];
    rest_username.resize(USERNAME_SIZE - username.len(), 0);
    let mut rest_email = vec![];
    rest_email.resize(EMAIL_SIZE - email.len(), 0);

    buf_username.copy_from_slice([username.as_bytes(), &rest_username].concat().as_slice());
    buf_email.copy_from_slice([email.as_bytes(), &rest_email].concat().as_slice());

    Row {
      id,
      username: buf_username,
      email: buf_email,
    }
  }

  pub fn ser(&self) -> [u8; ROW_SIZE] {
    let mut serialized = [0u8; ROW_SIZE];
    let id: [u8; ID_SIZE] = unsafe { transmute(self.id) };
    let username: [u8; USERNAME_SIZE] = unsafe { transmute(self.username) };
    let email: [u8; EMAIL_SIZE] = unsafe { transmute(self.email) };
    serialized.copy_from_slice([&id[..], &username[..], &email].concat().as_slice());
    serialized
  }

  pub fn de(source: [u8; ROW_SIZE]) -> Self {
    let username_offset = ID_SIZE + USERNAME_SIZE;
    let id = &source[0..ID_SIZE];
    let username = &source[ID_SIZE..username_offset];
    let email = &source[username_offset..];

    let mut buf_id = [0; ID_SIZE];
    let mut buf_username = [0; USERNAME_SIZE];
    let mut buf_email = [0; EMAIL_SIZE];
    buf_id.copy_from_slice(&id);
    buf_username.copy_from_slice(&username);
    buf_email.copy_from_slice(&email);

    let id: u32 = unsafe { transmute(buf_id) };
    let username: [u8; USERNAME_SIZE] = unsafe { transmute(buf_username) };
    let email: [u8; EMAIL_SIZE] = unsafe { transmute(buf_email) };
    Row {
      id,
      username,
      email,
    }
  }
}

impl fmt::Debug for Row {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let username = String::from_utf8_lossy(&self.username);
    let email = String::from_utf8_lossy(&self.email);
    write!(
      f,
      "Row {{ id: {}, username: {}, email: {} }}",
      self.id, username, email
    )
  }
}

#[derive(Debug)]
pub struct Table {
  pager: Pager,
  last_row: u32,
}

impl Table {
  pub fn open_db(disk: Option<String>) -> Self {
    Table {
      pager: Pager::open(disk),
      last_row: 0,
    }
  }

  pub fn insert(&mut self, row: Row) {
    let row = row.ser();
    let row_num = self.last_row as usize;
    let page_num = row_num / ROWS_PER_PAGE;

    self.last_row += 1;

    let mut page_empty = vec![];
    page_empty.resize(PAGE_SIZE, 0);
    let mut page = self
      .pager
      .get_page(page_num)
      .unwrap_or(page_empty)
      .to_owned();

    let row_offset = row_num % ROWS_PER_PAGE;
    let byte_offset = row_offset * ROW_SIZE;

    for i in byte_offset..byte_offset + ROW_SIZE {
      let idx_of_row = i - byte_offset;
      let el = row.get(idx_of_row).unwrap();
      page[i] = *el;
    }
    self.pager.flush_page(page_num, page).unwrap();
  }

  pub fn select(&mut self) -> Vec<Row> {
    let row_num = self.last_row as usize;
    let mut buf: Vec<Row> = vec![];
    for i in 0..row_num {
      let page_num = i / ROWS_PER_PAGE;
      let mut page = self.pager.get_page(page_num).unwrap();
      let row_offset = i % ROWS_PER_PAGE;
      let byte_offset = row_offset * ROW_SIZE;
      let row = page
        .drain(byte_offset..byte_offset + ROW_SIZE)
        .collect::<Vec<_>>();
      let mut buf_row = [0; ROW_SIZE];
      buf_row.copy_from_slice(&row);
      buf.push(Row::de(buf_row));
    }
    buf
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn db_by_timestamp(name: &str) -> Option<String> {
    if let Ok(ts) = SystemTime::now().duration_since(UNIX_EPOCH) {
      let ts = ts.as_secs();
      Some(format!("./tmp/{}.{}.db", name, ts))
    } else {
      unreachable!();
    }
  }

  impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
      self.id == other.id && self.username == other.username
        && String::from_utf8_lossy(&self.email) == String::from_utf8_lossy(&other.email)
    }

    fn ne(&self, other: &Self) -> bool {
      self.id != other.id || self.username != other.username
        || String::from_utf8_lossy(&self.email) != String::from_utf8_lossy(&other.email)
    }
  }

  #[test]
  fn test_ser() {
    let bytes_of_row = Row::new(
      1u32,
      "sample-user-name".to_owned(),
      "sample-email@user.com".to_owned(),
    );
    assert_eq!(bytes_of_row, Row::de(bytes_of_row.ser()));
    assert_eq!(bytes_of_row.ser().len(), ROW_SIZE);
  }

  #[test]
  fn test_insert() {
    let mut table = Table::open_db(db_by_timestamp("test_insert"));
    for i in 0..20 {
      table.insert(Row::new(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
      ));
    }
    assert_eq!(table.last_row, 20);
  }

  #[test]
  fn test_select() {
    let mut table = Table::open_db(db_by_timestamp("test_select"));
    let mut expects = vec![];
    for i in 0..20 {
      table.insert(Row::new(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
      ));
      expects.push(Row::new(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
      ));
    }
    assert_eq!(table.select(), expects);
  }
}
