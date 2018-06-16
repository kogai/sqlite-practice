use std::env::current_dir;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use table::PAGE_SIZE;

#[derive(Debug)]
pub struct Pager {
  file: File,
  seek_cursor: u64,
  size: usize,
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

  pub fn open() -> Self {
    let disk = Pager::get_disk();
    let file = match File::open(&disk) {
      Ok(file) => file,
      Err(_) => File::create(&disk).expect("Can not create database file."),
    };
    Pager {
      file,
      seek_cursor: 0,
      size: 0,
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

  pub fn set_len(&mut self, size: usize) {
    self.size = size;
  }

  pub fn len(&self) -> usize {
    self.size
  }
}
