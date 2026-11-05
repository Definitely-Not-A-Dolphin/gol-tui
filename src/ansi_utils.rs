use ansi_control_codes::control_sequences::{CUP, ED, ErasePage};

pub fn to_start() {
  print!("{}", CUP(0.into(), 0.into()));
}

pub fn erase_page() {
  print!("{}", ED(Some(ErasePage::BeginToEnd)));
}
