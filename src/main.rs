pub mod input;
pub mod user_sorted_list;
pub mod averaged_list;

use input::Input;
use user_sorted_list::UserSortedList;
use averaged_list::AveragedList;

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
  println!("{}", averaged_list);
}
