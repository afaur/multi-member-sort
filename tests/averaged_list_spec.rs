extern crate multi_member_sort;

#[cfg(test)]
mod user_sorted_list_test {
  use multi_member_sort::averaged_list::AveragedList;
  use multi_member_sort::user_sorted_list::UserSortedList;

  #[test]
  fn test_position() {
    let mut averaged_list = AveragedList::new();
    assert_eq!(vec!["fish tacos", "tabasco", "chips and guac", "50 cent tacos"], averaged_list.list);
    let adam = UserSortedList { list: vec![
      "chips and guac".to_string(),
      "tabasco".to_string(),
      "fish tacos".to_string(),
      "50 cent tacos".to_string(),
    ]};
    averaged_list.add_child(adam);
    let blaine = UserSortedList { list: vec![
      "chips and guac".to_string(),
      "fish tacos".to_string(),
      "50 cent tacos".to_string(),
      "tabasco".to_string(),
    ]};
    averaged_list.add_child(blaine);
    averaged_list.sort();
    assert_eq!(vec!["chips and guac", "fish tacos", "tabasco", "50 cent tacos"], averaged_list.list);
  }
}
