use std::collections::HashMap;

// const ID_SIZE: usize = 4;
// const USERNAME_SIZE: usize = 32;
// const EMAIL_SIZE: usize = 255;
// pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
pub const PAGE_SIZE: usize = 4096;
// const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

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
      &INTEGER => 4,
    }
  }
}

pub struct Row {
  data: Vec<u8>,
  definitions: HashMap<String, DataType>,
  row_size: usize,
  row_per_page: usize,
}

impl Row {
  pub fn new(id: u32, username: String, email: String) -> Self {
    use self::DataType::*;

    // Hardcoded definition of the row.
    let mut hs = HashMap::new();
    hs.insert("id".to_owned(), INTEGER);
    hs.insert("username".to_owned(), TEXT(32));
    hs.insert("email".to_owned(), TEXT(255));
    let row_size = hs.iter().fold(0, |acc, (_, v)| acc + v.size());
    let row_per_page = PAGE_SIZE / row_size;

    // let mut buf_username = [0; USERNAME_SIZE];
    // let mut buf_email = [0; EMAIL_SIZE];
    // let mut rest_username = vec![];
    // rest_username.resize(USERNAME_SIZE - username.len(), 0);
    // let mut rest_email = vec![];
    // rest_email.resize(EMAIL_SIZE - email.len(), 0);

    // buf_username.copy_from_slice([username.as_bytes(), &rest_username].concat().as_slice());
    // buf_email.copy_from_slice([email.as_bytes(), &rest_email].concat().as_slice());

    Row {
      data: vec![],
      definitions: hs,
      row_size,
      row_per_page,
    }
  }
}

// impl Row {
//   pub fn ser(&self) -> [u8; ROW_SIZE] {
//     // let mut serialized = [0u8; ROW_SIZE];
//     // let id: [u8; ID_SIZE] = unsafe { transmute(self.id) };
//     // let username: [u8; USERNAME_SIZE] = unsafe { transmute(self.username) };
//     // let email: [u8; EMAIL_SIZE] = unsafe { transmute(self.email) };
//     // serialized.copy_from_slice([&id[..], &username[..], &email].concat().as_slice());
//     // serialized
//     unimplemented!();
//   }

//   pub fn de(source: [u8; ROW_SIZE]) -> Result<Self, Error> {
//     //   let username_offset = ID_SIZE + USERNAME_SIZE;
//     //   let id = &source[0..ID_SIZE];
//     //   let username = &source[ID_SIZE..username_offset];
//     //   let email = &source[username_offset..];

//     //   let mut buf_id = [0; ID_SIZE];
//     //   let mut buf_username = [0; USERNAME_SIZE];
//     //   let mut buf_email = [0; EMAIL_SIZE];
//     //   buf_id.copy_from_slice(&id);
//     //   buf_username.copy_from_slice(&username);
//     //   buf_email.copy_from_slice(&email);

//     //   match (buf_username.get(0), buf_email.get(0)) {
//     //     (Some(y), Some(z)) if *y > 0 && *z > 0 => {
//     //       let id: u32 = unsafe { transmute(buf_id) };
//     //       let username: [u8; USERNAME_SIZE] = unsafe { transmute(buf_username) };
//     //       let email: [u8; EMAIL_SIZE] = unsafe { transmute(buf_email) };
//     //       Ok(Row {
//     //         id,
//     //         username,
//     //         email,
//     //       })
//     //     }
//     //     _ => Err(Error),
//     //   }
//     unimplemented!();
//   }
// }

// impl fmt::Debug for Row {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     let username = String::from_utf8_lossy(&self.username);
//     let email = String::from_utf8_lossy(&self.email);
//     write!(
//       f,
//       "Row {{ id: {}, username: {}, email: {} }}",
//       self.id, username, email
//     )
//   }
// }

// pub trait Serialize {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer;
// }

// pub trait Deserialize<'de>: Sized {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>;
// }

mod tests {
  use super::*;

  #[test]
  fn test_row_size() {
    println!("{:b}", 100u32);
    println!("{:?}", 1.0f32);
  }

}
