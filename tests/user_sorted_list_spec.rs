extern crate multi_member_sort;

#[cfg(test)]
mod user_sorted_list_test {
  use multi_member_sort::user_sorted_list::UserSortedList;

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
