use pager::Pager;
use row::{Definition, Row, PAGE_SIZE};

#[derive(Debug)]
pub struct Table<'a> {
  pager: Pager,
  last_row: u64,
  pub def: &'a Definition,
}

impl<'a> Table<'a> {
  pub fn open_db(disk: Option<String>, def: &'a Definition) -> Self {
    let mut pager = Pager::open(disk);
    let last_row = pager.rows(&def);
    Table {
      pager,
      last_row,
      def,
    }
  }

  pub fn insert(&mut self, row: Row) {
    let row = row.data;
    let row_num = self.last_row as usize;
    let page_num = row_num / self.def.row_per_page;

    self.last_row += 1;

    let mut page_empty = vec![];
    page_empty.resize(PAGE_SIZE, 0);
    let mut page = self
      .pager
      .get_page(page_num)
      .unwrap_or(page_empty)
      .to_owned();

    let row_offset = row_num % self.def.row_per_page;
    let byte_offset = row_offset * self.def.row_size;

    for i in byte_offset..byte_offset + self.def.row_size {
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
      let page_num = i / self.def.row_per_page;
      let mut page = self.pager.get_page(page_num).unwrap();
      let row_offset = i % self.def.row_per_page;
      let byte_offset = row_offset * self.def.row_size;
      let row = page
        .drain(byte_offset..byte_offset + self.def.row_size)
        .collect::<Vec<_>>();
      let mut buf_row = vec![0; self.def.row_size];
      buf_row.copy_from_slice(&row);
      if let Ok(row) = Row::de(&buf_row, &self.def) {
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

  #[test]
  fn test_ser() {
    let def = Definition::new();
    let bytes_of_row = Row::ser(
      1u32,
      "sample-user-name".to_owned(),
      "sample-email@user.com".to_owned(),
      &def,
    );
    println!("{:?}", Row::de(&bytes_of_row.data, &def).unwrap());
    assert_eq!(bytes_of_row, Row::de(&bytes_of_row.data, &def).unwrap());
    assert_eq!(bytes_of_row.data.len(), def.row_size);
  }

  #[test]
  fn test_insert() {
    let def = Definition::new();
    let mut table = Table::open_db(db_by_timestamp("test_insert"), &def);
    for i in 0..20 {
      table.insert(Row::ser(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
        &def,
      ));
    }
    assert_eq!(table.last_row, 20);
  }

  #[test]
  fn test_select() {
    let def = Definition::new();
    let mut table = Table::open_db(db_by_timestamp("test_select"), &def);
    let mut expects = vec![];
    for i in 0..20 {
      table.insert(Row::ser(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
        &def,
      ));
      expects.push(Row::ser(
        i,
        format!("sample-user-name-{}", i),
        format!("sample-user-name-{}@user.com", i),
        &table.def,
      ));
    }
    assert_eq!(table.select(), expects);
  }
}
