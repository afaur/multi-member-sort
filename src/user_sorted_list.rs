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
