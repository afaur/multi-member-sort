use std::io::{Result, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;

struct Input {}
impl Input {
  fn line(input: &mut String) -> Result<()> {
    try!(stdin().read_line(input));
    Ok(())
  }

  fn file(filename: &'static str, file_contents: &mut String) -> Result<()> {
    let mut file_pointer = try!(File::open(filename));
    try!(file_pointer.read_to_string(file_contents));
    Ok(())
  }

  fn number() -> i32 {
    let mut input = String::new();
    stdout().flush().ok();
    Input::line(&mut input).unwrap();

    return input.trim().parse::<i32>().unwrap();
  }
}

struct UserSortedList {
  list: Vec<String>,
}

impl UserSortedList {
  fn sort(&mut self) {
    self.list.sort_by(|a, b| {
      println!("1) {0}\n2) Equal\n3) {1}", a, b);
      let get_sort = Input::number();
      2.cmp(&get_sort)
    });
  }

  fn position(&self, needle: &String) -> usize {
    self.list.iter().position(|item| item == needle).unwrap()
  }
}

struct AveragedList {
  list: Vec<String>,
  child_lists: Vec<UserSortedList>,
}

impl AveragedList {
  fn new() -> AveragedList {
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

  fn add_child(&mut self, child_list: UserSortedList) {
    self.child_lists.push(child_list);
  }

  fn sort(&mut self) {
    let ref child_lists = self.child_lists;
    self.list.sort_by(|a, b| {
      let a_abs = child_lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(a)
      });
      let b_abs = child_lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(b)
      });
      b_abs.cmp(&a_abs)
    });
  }
}

fn main() {
  let mut averaged_list = AveragedList::new();

  print!("How many people? ");
  let list_count = Input::number();

  for _ in 0..list_count {
    let mut new_list = UserSortedList { list: averaged_list.list.clone() };
    new_list.sort();
    averaged_list.add_child(new_list);
  }

  averaged_list.sort();
}
