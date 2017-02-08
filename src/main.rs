use std::io;
use std::io::Read;
fn main() {
  let mut input = Vec::new();
  let stdin = io::stdin();
  let mut stdin = stdin.lock();
  match stdin.read_to_end(&mut input) {
    Ok(_) => {},
      Err(error) => println!("error: {}", error),
  }
  let f: u8 = '\n' as u8;
  let mut items : Vec<String> = input.split(|s| &f == s).map(|s| String::from_utf8(Vec::from(s)).unwrap() ).collect::<Vec<String>>();
  items.sort();
  for item in items.into_iter() {
    println!("{}", item);
  }
}
