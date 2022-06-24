/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use ::lolcat::*;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() -> Result<(), Box<dyn Error>> {
  let mut my_lolcat = Lolcat::new();
  println!("{:?}", my_lolcat);

  let file = File::open("Cargo.lock").expect("Error in reading file");
  let buffer_reader = BufReader::new(file);

  for (index, line) in buffer_reader.lines().enumerate() {
    let line = line.unwrap();
    println!("{}. {}", index + 1, my_lolcat.format_str(&line).to_string());
  }

  Ok(())
}

pub fn main3() -> Result<(), Box<dyn Error>> {
  let sample_string = fs::read_to_string("Cargo.lock")?;
  let mut my_lolcat = Lolcat::new();
  let output_collector = my_lolcat.format_str(&sample_string);
  println!("{}", output_collector.to_string());
  Ok(())
}

pub fn main2() {
  let sample_string = "Attack feet behind the couch destroy couch \
  flop over give attitude hide when guests come over hopped up on \
  goofballs hunt anything";

  let mut my_lolcat = Lolcat::new();

  let output_collector = my_lolcat.format_str(sample_string);
  println!("{}", output_collector.to_string());

  let output_collector = my_lolcat.format_str(sample_string);
  println!("{}", output_collector.to_string());
}
