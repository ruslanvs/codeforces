use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).unwrap();

  let weight = buffer
    .trim()
    .parse::<u32>()
    .unwrap();

  if weight % 2 == 0 && weight > 2 {
    println!("YES");
  } else {
    println!("NO");
  }
  Ok(())
}
