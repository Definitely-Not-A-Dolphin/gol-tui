use ansi_control_codes::control_sequences::CNL;
use std::io::{Write, stdout};

pub fn to_pos(line: u16, column: u16) {
  print!("\x1b[{line};{column}H");
}

pub fn new_line() {
  print!("{}", CNL(None));
}

pub fn erase_page() {
  print!("\x1b[K\x1b[1J");
  stdout().flush().unwrap();
}
