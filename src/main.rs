use std::{io::{self, Write, Read}, fs::File};

fn main() {
    println!("Hello, world!");
    println!("Please enter the README file or root-directory");
    print!(">> ");
    let input = input();
    println!("{}", open(&input));
}

fn input() -> String {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer.trim().to_string()
}

fn open(path: &str) -> String {
    let mut file = File::open(path).expect(format!("Failed to open file: {}", path).as_str());
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");
    buffer
}