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

fn get_list(list: &mut Vec<String>) {
  let mut file_contents = String::new();
  read_file("./data/list.txt", &mut file_contents).unwrap();
  for item in file_contents.trim().split("\n") {
    list.push(item.to_string());
  }
}

fn user_sort_list(list: &mut Vec<String>) {
  list.sort_by(|a, b| {
    println!("1) {0}\n2) Equal\n3) {1}", a, b);
    let get_sort = get_number_input();
    2.cmp(&get_sort)
  });
}

fn main() {
  let mut original_list:Vec<String> = vec![];
  get_list(&mut original_list);

  print!("How many people? ");
  let list_count = get_number_input();
  let mut lists:Vec<Vec<String>> = vec![];

  for _ in 0..list_count {
    let mut new_list = original_list.clone();
    user_sort_list(&mut new_list);
    lists.push(new_list);
  }

  original_list.sort_by(|a, b| {
    let a_abs = lists.iter().fold(0, |memo, list| {
      memo + list.iter().position(|item| item == a).unwrap()
    });
    let b_abs = lists.iter().fold(0, |memo, list| {
      memo + list.iter().position(|item| item == b).unwrap()
    });
    b_abs.cmp(&a_abs)
  });

  println!("Averaged list: {:?}", original_list);
}
