use std::io::Read;

mod tm1;
mod tm2;
mod tm3;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut read: Box<dyn std::io::Read> = match args.iter().nth(1) {
        Some(a) => Box::new(std::fs::File::open(a).unwrap()),
        None => Box::new(std::io::stdin()),
    };
    loop {
        let mut s = String::new();
        read.read_to_string(&mut s).unwrap();
        for token in s.split_whitespace() {
            println!("{}", token);
        }
    }
}
