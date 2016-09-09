extern crate multi_member_sort;

#[cfg(test)]
mod user_sorted_list_test {
  use multi_member_sort::averaged_list::AveragedList;
  use multi_member_sort::user_sorted_list::UserSortedList;

  #[test]
  fn test_position() {
    let mut averaged_list = AveragedList::new();
    let list = averaged_list.list.clone();
    let adam = UserSortedList { list: vec![
      list.get(2).unwrap().clone(),
      list.get(1).unwrap().clone(),
      list.get(0).unwrap().clone(),
      list.get(3).unwrap().clone(),
    ]};
    averaged_list.add_child(adam);
    let blaine = UserSortedList { list: vec![
      list.get(2).unwrap().clone(),
      list.get(0).unwrap().clone(),
      list.get(3).unwrap().clone(),
      list.get(1).unwrap().clone(),
    ]};
    averaged_list.add_child(blaine);
    averaged_list.sort();
    assert_eq!(vec![
      list.get(2).unwrap().clone(),
      list.get(0).unwrap().clone(),
      list.get(1).unwrap().clone(),
      list.get(3).unwrap().clone(),
    ], averaged_list.list)
  }
}
