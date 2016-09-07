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

fn get_count() -> i32 {
  let mut input = String::new();
  stdout().flush().ok();
  read_line(&mut input).unwrap();

  return input.trim().parse::<i32>().unwrap();
}

fn main() {
  let mut file_contents = String::new();
  read_file("./data/list.txt", &mut file_contents).unwrap();
  let mut original_list:Vec<&str> = file_contents.trim().split("\n").collect();
  println!("{:?}", original_list);

  print!("How many people? ");
  let list_count = get_count();
  let mut lists:Vec<Vec<&str>> = vec![];

  for x in 0..list_count {
    let mut new_list = original_list.clone();
    println!("Person {} go!", x);
    new_list.sort_by(|a, b| {
      println!("1) {0}\n2) Equal\n3) {1}", a, b);
      let get_sort = get_count();
      2.cmp(&get_sort)
    });
    lists.push(new_list);
  }

  original_list.sort_by(|&a, &b| {
    let a_abs = lists.iter().fold(0, |memo, list| {
      let index = list.iter().position(|&item| item == a).unwrap();
      index + memo
    });
    let b_abs = lists.iter().fold(0, |memo, list| {
      let index = list.iter().position(|&item| item == b).unwrap();
      index + memo
    });
    b_abs.cmp(&a_abs)
  });

  println!("{:?}", original_list);
}
