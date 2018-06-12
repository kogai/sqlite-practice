use std::fmt;
use std::mem::transmute;

/*
id: integer 4bytes
username: varchar(32) 32bytes
email: varchar(255) 255bytes
NOTE: For simplicity, we hardcode a user schema as above.
*/

const ID_SIZE: usize = 4;
// NOTE: We can eliminate useless spaces of 4bytes per char.
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

pub struct Row {
  id: u32,
  username: [char; USERNAME_SIZE],
  email: [char; EMAIL_SIZE],
}

impl Row {
  pub fn new(id: u32, username: String, email: String) -> Self {
    let username = username.chars().enumerate().fold(
      ['\u{0}'; USERNAME_SIZE],
      |mut acc, (i, c)| {
        acc[i] = c;
        acc
      },
    );
    let email = email
      .chars()
      .enumerate()
      .fold(['\u{0}'; EMAIL_SIZE], |mut acc, (i, c)| {
        acc[i] = c;
        acc
      });
    Row {
      id,
      username,
      email,
    }
  }

  pub fn ser(&self) -> [u8; ID_SIZE + (USERNAME_SIZE * 4) + (EMAIL_SIZE * 4)] {
    let mut serialized = [0u8; ID_SIZE + (USERNAME_SIZE * 4) + (EMAIL_SIZE * 4)];
    let id: [u8; ID_SIZE] = unsafe { transmute(self.id) };
    let username: [u8; USERNAME_SIZE * 4] = unsafe { transmute(self.username) };
    let email: [u8; EMAIL_SIZE * 4] = unsafe { transmute(self.email) };
    serialized.copy_from_slice([&id[..], &username[..], &email].concat().as_slice());
    serialized
  }

  pub fn de(source: [u8; ID_SIZE + (USERNAME_SIZE * 4) + (EMAIL_SIZE * 4)]) -> Self {
    let username_offset = ID_SIZE + (USERNAME_SIZE * 4);
    let id = &source[0..ID_SIZE];
    let username = &source[ID_SIZE..username_offset];
    let email = &source[username_offset..];
    // let username = self.username.iter().collect::<String>();
    // let email = self.email.iter().collect::<String>();
    // let id: u32 = unsafe { transmute(id) };
    println!("{:?}", id);
    println!("{:?}", username);
    println!("{:?}", email);
    unimplemented!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let username = self.username.iter().collect::<String>();
      let email = self.email.iter().collect::<String>();
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
        && self.email.iter().collect::<String>() == other.email.iter().collect::<String>()
    }

    fn ne(&self, other: &Self) -> bool {
      self.id != other.id || self.username != other.username
        || self.email.iter().collect::<String>() == other.email.iter().collect::<String>()
    }
  }

  #[test]
  fn test_ser() {
    let bytes_of_row = Row::new(
      1u32,
      "sample-user-name".to_owned(),
      "sample-email@user.com".to_owned(),
    );

    // println!(
    //   "{:?}",
    //   unsafe { transmute::<[char; 4], [u8; 16]>(['a', 'b', 'c', 'd']) }.to_vec()
    // );
    assert_eq!(bytes_of_row, Row::de(bytes_of_row.ser()));
  }
}
