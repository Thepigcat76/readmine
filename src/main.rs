use std::{
    env::current_dir,
    fs::{self, File},
    io::{self, Write},
};

#[derive(Debug)]
struct READMELayout {
    head: String,
    content_table: Vec<String>,
    contributors: Vec<String>,
}

impl READMELayout {
    fn new(head: String, content_table: Vec<String>, contributors: Vec<String>) -> Self {
        Self {
            head,
            content_table,
            contributors,
        }
    }

    fn write(&mut self, file: &mut File) {
        let mut buffer: Vec<String> = vec![format!("# {}\n\n", self.head)];

        if self.content_table[0] == "__default__" && &self.content_table.len() == &1 {
            self.content_table = vec![
                "Introduction".to_string(),
                "Installation".to_string(),
                "Documentation".to_string(),
                "Contributors".to_string(),
            ]
        }

        for entry in &self.content_table {
            buffer.push(format!("- [{}](#{})\n\n", entry, entry.to_lowercase()))
        }

        for i in 0..self.content_table.len() {
            if self.content_table[i] != "Contributors" {
                buffer.push(format!(
                    "## {}\n\nEnter content here\n\n",
                    self.content_table[i]
                ))
            }
        }

        if &self.contributors.len() > &0 {
            buffer.push("## Contributors\n\n".to_string());
            for contributor in &self.contributors {
                buffer.push(format!(
                    "- [{}](https://github.com/{})\n",
                    contributor, contributor
                ))
            }
        }

        let buffer_string: String = buffer.join("");

        file.write_all(buffer_string.as_bytes())
            .expect("Failed to write to file");
    }
}

fn main() {
    let root_dir = get_root_dir_upcase();
    let cur_dir = current_dir().expect("Failed to access current dir");
    println!("Please enter the README file path or root-directory path\nor leave empty if correct root-dir is already selected");
    let path_str = msg_input(cur_dir.to_str().expect("Failed to unwrap"));
    let mut file = open(&path_str);
    println!("Enter the content table. Seperate entries with COMMA.\nEnter __default__ to use the default Layout");
    let content_table_raw = msg_input(">> ");
    let content_table = convert_string_to_vec(&content_table_raw);
    println!("Enter contributors. Leave empty to skip this step");
    let contributors_raw = msg_input(">> ");
    let contributors = convert_string_to_vec(&contributors_raw);
    println!("--DEBUG--");
    println!("Root dir: {root_dir}");
    println!("Cur dir: {}", cur_dir.to_str().expect("Failed to unwrap"));
    let mut layout = READMELayout::new(root_dir, content_table, contributors);
    println!("{:?}", layout);
    layout.write(&mut file);
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
    print!("{}\\", msg);
    input()
}

fn open(path: &str) -> File {
    let full_path: String = format!(
        "{}\\{}\\README.md",
        current_dir()
            .expect("Failed to acces current dir")
            .to_str()
            .expect("Failed to unwrap path buf"),
        path
    );

    match fs::remove_file(&full_path) {
        Ok(_) => (),
        Err(_) => (),
    }
    let file = File::create(&full_path).expect("Failed to create File");
    file
}

fn get_root_dir() -> String {
    let cur_dir = current_dir().expect("Failed to access current directory");
    let cd_chars: Vec<char> = cur_dir
        .to_str()
        .expect("Failed to unwrap")
        .chars()
        .collect();
    let mut root_dir_raw = String::new();
    for c in cd_chars.iter().rev() {
        if c != &'\\' {
            let _ = &root_dir_raw.push(*c);
        } else {
            let _ = &root_dir_raw;
            break;
        }
    }
    let root_dir: String = root_dir_raw.chars().rev().collect();
    root_dir
}

fn get_root_dir_upcase() -> String {
    let root_dir = get_root_dir();
    capitalize_first_letter(&root_dir)
}

fn capitalize_first_letter(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let first_upper = first_char.to_uppercase();
        let mut modified = String::with_capacity(input.len());
        modified.push_str(&first_upper.to_string());
        modified.push_str(&input[first_char.len_utf8()..]);
        modified
    } else {
        input.to_string()
    }
}

/// creates a vector from a list in a string ("Hello, World, !") -> vec!["Hello", "World", "!"]
fn convert_string_to_vec(input: &str) -> Vec<String> {
    let result: Vec<&str> = input.split(',').collect();
    let string_result: Vec<String> = result.iter().map(|&s| s.trim().to_string()).collect();
    string_result
}
