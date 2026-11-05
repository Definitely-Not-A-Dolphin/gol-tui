use crate::ansi_utils::{erase_page, to_start};
use terminal_size::{Height, Width, terminal_size};

mod ansi_utils;

fn main() {
  let size = terminal_size();

  if let Some((Width(w), Height(h))) = size {
    println!("Your terminal is {} cols wide and {} lines tall", w, h);
  } else {
    println!("Unable to get terminal size, sowwy :(");
    std::process::exit(1);
  }

  to_start();
  print!("Haiii ;3");
  erase_page();
  to_start();
  print!("Haiii ;3");
}
