use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer)?;
  let lines = buffer.lines();
  for (i, line) in lines.enumerate() {
    if i > 0 {
      let length = line.len();
      if length > 10 {
        let length_minus_two = length - 2;
        let first_char = line.chars().nth(0).unwrap();
        let last_char = line.chars().nth(length - 1).unwrap();
        println!("{}{}{}", first_char, length_minus_two, last_char);
      } else {
        println!("{}", line);
      }
    }
  }
  Ok(())
}