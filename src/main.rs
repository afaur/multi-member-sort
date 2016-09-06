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

fn get_list_count() -> i32 {
  let mut input = String::new();
  print!("How many people? ");
  stdout().flush().ok();
  read_line(&mut input).unwrap();

  return input.trim().parse::<i32>().unwrap();
}

fn main() {
  let mut file_contents = String::new();
  read_file("./data/list.txt", &mut file_contents).unwrap();
  let original_list:Vec<&str> = file_contents.trim().split("\n").collect();
  println!("{:?}", original_list);

  let list_count = get_list_count();
  let mut lists:Vec<Vec<&str>> = vec![];

  for _ in 0..list_count {
    let mut new_list = original_list.clone();
    new_list.sort_by(|a, b| {
      a.cmp(b)
    });
    lists.push(new_list);
  }
  println!("{:?}", lists);
}
