pub mod input;
pub mod user_sorted_list;
pub mod averaged_list;

extern crate ansi_term;

use input::Input;
use user_sorted_list::UserSortedList;
use averaged_list::AveragedList;
use ansi_term::Colour::Green;
use ansi_term::Colour::Blue;

fn main() {
  let mut averaged_list = AveragedList::new();

  print!("{}", Green.paint("How many people? "));
  let list_count = Input::number();

  for id in 0..list_count {
    println!("{}", Blue.paint(format!("Person {} starts:", id)));
    let mut new_list = UserSortedList { list: averaged_list.list.clone() };
    new_list.sort();
    averaged_list.add_child(new_list);
  }

  averaged_list.sort();
  print!("{}", Green.paint("RESULTS:"));
  println!("{}", averaged_list);
}
