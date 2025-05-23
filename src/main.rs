use std::io::Read;

mod tm1;
mod tm2;
mod tm3;

fn main() {
    let mut stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_to_string(&mut s).unwrap();
    for token in s.split_whitespace() {
        println!("{}", token);
    }
}
