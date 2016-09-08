use std::io::{Result, stdin, stdout};
use std::fs::File;
use std::io::prelude::*;

pub struct Input {}
impl Input {
  pub fn line(input: &mut String) -> Result<()> {
    try!(stdin().read_line(input));
    Ok(())
  }

  pub fn file(filename: &'static str, file_contents: &mut String) -> Result<()> {
    let mut file_pointer = try!(File::open(filename));
    try!(file_pointer.read_to_string(file_contents));
    Ok(())
  }

  pub fn number() -> i32 {
    let mut input = String::new();
    stdout().flush().ok();
    Input::line(&mut input).unwrap();
    return input.trim().parse::<i32>().unwrap();
  }
}
