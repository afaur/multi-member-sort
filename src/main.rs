use std::io::{Result, stdin, stdout};
use std::io::prelude::*;
use std::fs::File;

fn read_line(input: &mut String) -> Result<()> {
  try!(stdin().read_line(input));
  Ok(())
}

fn read_file(filename: &'static str, file_contents: &mut String) -> Result<()> {
  let mut f = try!(File::open(filename));
  try!(f.read_to_string(file_contents));
  Ok(())
}

fn get_number_input() -> i32 {
  let mut input = String::new();
  stdout().flush().ok();
  read_line(&mut input).unwrap();

  return input.trim().parse::<i32>().unwrap();
}

struct UserSortedList {
  list: Vec<String>,
}

impl UserSortedList {
  fn sort(&mut self) {
    self.list.sort_by(|a, b| {
      println!("1) {0}\n2) Equal\n3) {1}", a, b);
      let get_sort = get_number_input();
      2.cmp(&get_sort)
    });
  }

  fn position(&self, needle: &String) -> usize {
    self.list.iter().position(|item| item == needle).unwrap()
  }
}

struct AveragedList {
  list: Vec<String>,
}

impl AveragedList {
  fn new() -> AveragedList {
    let mut original_list:Vec<String> = vec![];
    let mut file_contents = String::new();
    read_file("./data/list.txt", &mut file_contents).unwrap();
    for item in file_contents.trim().split("\n") {
      original_list.push(item.to_string());
    }
    AveragedList { list: original_list }
  }

  fn sort(&mut self) {
    print!("How many people? ");
    let list_count = get_number_input();
    let mut lists:Vec<UserSortedList> = vec![];

    for _ in 0..list_count {
      let mut new_list = UserSortedList { list: self.list.clone() };
      new_list.sort();
      lists.push(new_list);
    }

    self.list.sort_by(|a, b| {
      let a_abs = lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(a)
      });
      let b_abs = lists.iter().fold(0, |memo, user_list| {
        memo + user_list.position(b)
      });
      b_abs.cmp(&a_abs)
    });

    println!("Averaged list: {:?}", self.list);
  }
}

fn main() {
  let mut averaged_list = AveragedList::new();
  averaged_list.sort();
}
