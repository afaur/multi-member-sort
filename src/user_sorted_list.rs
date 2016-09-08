use input::Input;

pub struct UserSortedList {
  pub list: Vec<String>,
}

impl UserSortedList {
  pub fn sort(&mut self) {
    self.list.sort_by(|a, b| {
      println!("1) {0}\n2) Equal\n3) {1}", a, b);
      let get_sort = Input::number();
      2.cmp(&get_sort)
    });
  }

  pub fn position(&self, needle: &String) -> usize {
    self.list.iter().position(|item| item == needle).unwrap()
  }

}

#[cfg(test)]
mod user_sorted_list_test {
  use super::*;

  #[test]
  fn test_position() {
    let user_sorted_list = UserSortedList {
      list: vec![
        "tiny".to_string(),
        "taco".to_string(),
        "team".to_string()
      ]
    };
    assert_eq!(0, user_sorted_list.position(&"tiny".to_string()));
    assert_eq!(1, user_sorted_list.position(&"taco".to_string()));
    assert_eq!(2, user_sorted_list.position(&"team".to_string()));
  }
}
