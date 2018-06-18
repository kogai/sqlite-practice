use pager::Pager;
use row::{Definition, Row};

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
pub const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

#[derive(Debug)]
pub struct Table {
  pager: Pager,
  last_row: u64,
}

impl Table {
  pub fn open_db(disk: Option<String>) -> Self {
    let mut pager = Pager::open(disk);
    let last_row = pager.rows();
    Table { pager, last_row }
  }

  pub fn insert(&mut self, row: Row) {
    let row = row.data;
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

  pub fn select(&mut self, def: &Definition) -> Vec<Row> {
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
      let mut buf_row = vec![0; ROW_SIZE];
      buf_row.copy_from_slice(&row);
      if let Ok(row) = Row::de(buf_row, def) {
        buf.push(row);
      }
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
    assert_eq!(bytes_of_row, Row::de(bytes_of_row.ser()).unwrap());
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
