#![allow(clippy::print_with_newline)]
use crate::ansi_utils::{erase_page, new_line, to_pos};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{Read, Write, stdout};
use terminal_size::{Height, Width, terminal_size};

mod ansi_utils;

fn main() {
  enable_raw_mode().unwrap();
  erase_page();

  let (width, height) = match terminal_size() {
    Some((Width(w), Height(h))) => (w, h),
    None => std::process::exit(1),
  };

  let mut field = vec![vec![false; width as usize]; height as usize];
  let (mut cur_line, mut cur_column) = (1, 1);

  (&mut field)[6][12] = true;

  loop {
    erase_page();
    to_pos(1, 1);

    for row in &field {
      for element in row {
        if *element {
          print!("6");
        } else {
          print!(" ");
        }
      }
    }

    stdout().flush().unwrap();

    let mut buf: [u8; 1] = [0];
    std::io::stdin().read_exact(&mut buf).unwrap();
    let buf = buf[0];

    match buf as char {
      'q' => break,
      '\u{001bA}' if cur_line != 1 => cur_line -= 1,
      '\u{001bB}' if cur_line != height => cur_line += 1,
      '\u{001bC}' if cur_column != 1 => cur_column -= 1,
      '\u{001bD}' if cur_column != width => cur_column += 1,
      _ => (),
    };

    (&mut field)[cur_line as usize][cur_column as usize] = true;
  }

  erase_page();
  to_pos(1, 1);
  disable_raw_mode().unwrap();
}
