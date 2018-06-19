use row::{Definition, Row};
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use table::{PAGE_SIZE, ROW_SIZE};

#[derive(Debug)]
pub struct Pager {
  file: File,
  seek_cursor: u64,
}

impl Pager {
  fn get_disk() -> PathBuf {
    current_dir()
      .map(|mut d| {
        d.push("tmp/sqlite.db");
        d
      })
      .expect("Getting current directory is failed.")
  }

  pub fn open(disk: Option<String>) -> Self {
    let path_of_disk = match disk {
      Some(path) => PathBuf::from(path),
      None => Pager::get_disk(),
    };
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(&path_of_disk)
      .unwrap();
    Pager {
      file,
      seek_cursor: 0,
    }
  }

  pub fn get_page<'a>(&mut self, page_num: usize) -> Result<Vec<u8>, io::Error> {
    let offset = (PAGE_SIZE * page_num) as u64;
    let mut buf = [0; PAGE_SIZE];
    self.file.seek(SeekFrom::Start(offset))?;
    self.file.read_exact(&mut buf)?;
    Ok(buf[..].to_vec())
  }

  pub fn flush_page(&mut self, page_num: usize, data: Vec<u8>) -> Result<(), io::Error> {
    let offset = (PAGE_SIZE * page_num) as u64;
    self.file.seek(SeekFrom::Start(offset))?;
    self.file.write(&data)?;
    Ok(())
  }

  // NOTE: Compromise O(n) liner counting.
  pub fn rows(&mut self) -> u64 {
    let mut offset = 0;
    let mut is_fill = true;
    let mut row_num = 0;
    let mut buf = [0; ROW_SIZE];
    let def = Definition::new();
    while is_fill {
      let file_size = self.file.metadata().map(|m| m.len()).unwrap() as usize;
      if file_size < PAGE_SIZE {
        is_fill = false;
        continue;
      }
      self.file.seek(SeekFrom::Start(offset)).unwrap();
      self.file.read_exact(&mut buf).unwrap();
      if let Ok(_) = Row::de(&buf.to_vec(), &def) {
        offset += ROW_SIZE as u64;
        row_num += 1;
      } else {
        is_fill = false;
      }
    }
    row_num
  }
}
