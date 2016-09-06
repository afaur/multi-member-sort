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

fn main() {
  let mut input = String::new();
  print!("How many people? ");
  stdout().flush().ok();
  read_line(&mut input).unwrap();

  let list_count = input.trim().parse::<i32>().ok().unwrap();
  println!("{:?}", list_count);

  let mut file_contents = String::new();
  read_file("./data/list.txt", &mut file_contents).unwrap();
  println!("{:?}", file_contents.lines());
}
