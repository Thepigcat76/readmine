use std::{
    env::current_dir,
    fs::File,
    io::{self, Read, Write}, ops::Sub,
};

struct FileLayout {
    head: String,
    content_table: Vec<String>,
}

impl FileLayout {
    fn new(head: String, content_table: Vec<String>) -> Self {
        Self { head, content_table }
    }
}

fn main() {
    let cur_dir = current_dir().expect("Failed to access current directory");
    let trimmed_dir = cur_dir.to_str().expect("Failed to unwrap").trim_matches('\'');
    let dir_chars: Vec<char> = trimmed_dir.chars().collect();
    let mut root_name_raw = String::new();
    for c in dir_chars.iter().rev() {
        if c != &'\\' {
            let _ = &root_name_raw.push(*c);
        } else {
            let _ = &root_name_raw;
            break;
        }
    }
    let root_name: String = root_name_raw.chars().rev().collect();
    println!("{}", root_name);
    println!("Please enter the README file or root-directory \nor press ENTER if correct root-dir is already selected");
    let path_str = msg_input(trimmed_dir);
    let mut file = open(&path_str);
    println!("DEBUG, File content: {}", read(&mut file));
    let head_chars: Vec<char> = path_str.chars().collect();
    let mut head = String::new();
    for i in 1..head_chars.len() {
        head = head_chars[0].to_uppercase().to_string();
        head.push(head_chars[i]);
    }
    println!("{}", head);
    let content_table = Vec::new();
    FileLayout::new(head, content_table);
}

fn input() -> String {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer.trim().to_string()
}

fn msg_input(msg: &str) -> String {
    io::stdout().flush().expect("Failed to flush stdout");
    let mut buffer = String::new();
    print!("{}", msg);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer.trim().to_string()
}

fn open(path: &str) -> File {
    let full_path: String = format!(
        "{}\\{}",
        current_dir()
            .expect("Failed to acces current dir")
            .to_str()
            .expect("Failed to unwrap path buf"),
        path
    );
    match File::open(&full_path) {
        Ok(file) => file,
        _ => File::open(format!("{}\\README.md", full_path)).expect(format!("Failed to open file: {}\\README.md", full_path).as_str()),
    }
}

fn write_readme(file: &mut File) {
    
}

fn read(file: &mut File) -> String {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Failed to read file");
    buffer
}
