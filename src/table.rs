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

  pub fn ser(&self) -> [u8; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE] {
    let mut serialized = [0u8; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE];
    let id: [u8; ID_SIZE] = unsafe { transmute(self.id) };
    let username: [u8; USERNAME_SIZE] = unsafe { transmute(self.username) };
    let email: [u8; EMAIL_SIZE] = unsafe { transmute(self.email) };
    serialized.copy_from_slice([&id[..], &username[..], &email].concat().as_slice());
    serialized
  }

  pub fn de(source: [u8; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE]) -> Self {
    let username_offset = ID_SIZE + (USERNAME_SIZE);
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

#[cfg(test)]
mod tests {
  use super::*;

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
    assert_eq!(
      bytes_of_row.ser().len(),
      ID_SIZE + USERNAME_SIZE + EMAIL_SIZE
    );
  }
}