pub struct Index {
  property: String,
}

impl Index {
  pub fn new(property: String) -> Index {
    Index { property: property }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn can_build() {
    let index = Index::new("favorite".to_string());
    assert_eq!("favorite", index.property);
  }

}
