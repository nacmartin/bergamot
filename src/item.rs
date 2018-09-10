use std::collections::HashMap;
use std::mem::size_of;

pub struct Item {
  id: u32,
  data: Vec<u8>,
  index: HashMap<String, u32>,
}

impl Item {
  pub fn new(id: u32) -> Item {
    Item {
      id: id,
      data: Vec::new(),
      index: HashMap::new(),
    }
  }

  pub fn add(&mut self, field: &mut Field) {
    let position = field.data.len() as u32;
    self.data.append(&mut field.data);
    self.index.insert(field.name.clone(), position);
  }
}

enum Type {
  Boolean,
  Text,
  Integer,
}

pub struct Field {
  name: String,
  data: Vec<u8>,
  data_type: Type,
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn can_build() {
    let item = Item::new(23);
    assert_eq!(0, item.data.len());
  }

  #[test]
  fn can_add_boolean() {
    let mut item = Item::new(23);
    let mut field = Field {
      name: "sportive".to_string(),
      data_type: Type::Boolean,
      data: vec![1],
    };
    item.add(&mut field);
    assert_eq!(1, item.data.len());
    assert_eq!(1, item.index.len());
  }

  #[test]
  fn can_add_integer() {
    let mut item = Item::new(23);
    let data: u32 = 42;
    let mut field = Field {
      name: "calories".to_string(),
      data_type: Type::Integer,
      data: u32_to_bytes(data).to_vec(),
    };
    item.add(&mut field);
    assert_eq!(4, item.data.len());
    assert_eq!(1, item.index.len());
  }
}

use std::mem::transmute;

fn u32_to_bytes(wat: u32) -> [u8; 4] {
  unsafe { transmute::<u32, [u8; 4]>(wat) }
}
//fn from_bytes<T: std::marker::Sized>(bytes: [u8; std::mem::size_of::<T>()]) -> T {
//  unsafe { std::mem::transmute(bytes) }
//}
