mod index;
mod item;

use item::Item;

pub struct Db {
  items: Vec<item::Item>,
}

impl Db {
  pub fn new() -> Db {
    Db { items: Vec::new() }
  }
  pub fn build(&mut self) {
    self.items = Vec::new();
  }

  pub fn len(self) -> usize {
    self.items.len()
  }

  pub fn add(&mut self, item: Item) {
    self.items.push(item);
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use item::Item;

  #[test]
  fn can_build() {
    let db = Db::new();
    assert_eq!(0, db.len());
  }

  #[test]
  fn can_add() {
    let mut db = Db::new();
    db.add(Item::new(32));
    assert_eq!(1, db.len());
  }
}
