use std::fmt;
/*
id: integer 4bytes
username: varchar(32) 32bytes
email: varchar(255) 255bytes
NOTE: For simplicity, we hardcode a user schema as above.
*/

const ID_SIZE: usize = 4;
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
// const ID_OFFSET: usize = 0;
// const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
// const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

pub struct Row {
  id: u32,
  username: [char; USERNAME_SIZE],
  email: [char; EMAIL_SIZE],
}
// impl<'a, 'b, A: $Bound, B> PartialEq<$Lhs> for $Rhs where B: PartialEq<A> {
//     #[inline]
//     #[inline]
// }
// /**
// * void serialize_row(Row* source, void* destination) {
// +  memcpy(destination + ID_OFFSET, &(source->id), ID_SIZE);
// +  memcpy(destination + USERNAME_OFFSET, &(source->username), USERNAME_SIZE);
// +  memcpy(destination + EMAIL_OFFSET, &(source->email), EMAIL_SIZE);
// +}
// +
// +void deserialize_row(void* source, Row* destination) {
// +  memcpy(&(destination->id), source + ID_OFFSET, ID_SIZE);
// +  memcpy(&(destination->username), source + USERNAME_OFFSET, USERNAME_SIZE);
// +  memcpy(&(destination->email), source + EMAIL_OFFSET, EMAIL_SIZE);
// +}
//  */

impl Row {
  pub fn new(id: u32, username: String, email: String) -> Self {
    unimplemented!();
  }

  pub fn ser(&self) -> [u8; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE] {
    unimplemented!();
  }

  pub fn de(source: [u8; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE]) -> Self {
    unimplemented!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // fmt::Debug::fmt(&&self[..], f)
      unimplemented!();
    }
  }

  impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
      // let is_same_emai = self.email.iter().all;
      self.id == other.id && self.username == other.username
      //  && self.email.iter()
      //  && self.email == other.email
    }
    fn ne(&self, other: &Self) -> bool {
      unimplemented!();
      // self[..] != other[..]
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
  }
}
