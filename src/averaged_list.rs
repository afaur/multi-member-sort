use std::fmt::{Result, Formatter, Display};
use user_sorted_list::UserSortedList;
use input::Input;

pub struct AveragedList {
  pub list: Vec<String>,
  child_lists: Vec<UserSortedList>,
}

impl Display for AveragedList {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self.list)
  }
}

impl AveragedList {
  pub fn new() -> AveragedList {
    let mut original_list:Vec<String> = vec![];
    let mut file_contents = String::new();
    Input::file("./data/list.txt", &mut file_contents).unwrap();
    for item in file_contents.trim().split("\n") {
      original_list.push(item.to_string());
    }
    AveragedList {
      list: original_list,
      child_lists: vec![],
    }
  }

  pub fn add_child(&mut self, child_list: UserSortedList) {
    self.child_lists.push(child_list);
  }

  pub fn sort(&mut self) {
    let ref child_lists = self.child_lists;
    self.list.sort_by(|a, b| {
      let a_abs = child_lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(a)
      });
      let b_abs = child_lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(b)
      });
      a_abs.cmp(&b_abs)
    });
  }
}
