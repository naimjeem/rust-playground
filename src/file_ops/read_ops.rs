

use std::fs::File;
use std::io::prelude::*;

pub fn read_file() {
  let mut file = File::open("./Cargo.toml").expect("Can't open file");
  let mut contents = String::new();

  file.read_to_string(&mut contents).expect("Cannot read file");

  println!("File Contents\n\n {}", contents);
}