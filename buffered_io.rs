use std::io::BufferedReader;
use std::io;

fn main() {
    let mut reader = BufferedReader::new(io::stdin());
    let input = reader.read_line().unwrap();
    println!("You typed: {}.", input);
}
