use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_line(input: &mut String) -> io::Result<()> {
  try!(io::stdin().read_line(input));
  Ok(())
}

fn read_file(filename: &'static str, file_contents: &mut String) -> io::Result<()> {
  let mut f = try!(File::open(filename));
  try!(f.read_to_string(file_contents));
  Ok(())
}

fn main() {
  let mut input = String::new();
  read_line(&mut input).unwrap();
  println!("{:?}", input);

  let mut file_contents = String::new();
  read_file("./data/list.txt", &mut file_contents).unwrap();
  println!("{:?}", file_contents.lines());
}
