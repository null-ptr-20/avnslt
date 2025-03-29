mod colors;
mod ui;
mod file;

use std::io::{self, Write};

use file::Summary;

pub fn prompt_create_file() -> file::File {
    println!("Please input the fields to start a file: ");

    let title = prompt_scan("Title: ");
    let date = prompt_scan("Date: ");
    let body = prompt_scan("Body: ");

    let new_text_file: file::File = file::File::build(title, date, body);
    
    println!("{}", new_text_file.summarize());

    new_text_file
}

pub fn scan_input(user_input: &mut String) -> &str {
    io::stdin().read_line(user_input).expect("no input");

    user_input.trim()
}

pub fn prompt_scan(output: &str) -> String {
    println!("{}", output);

    print!("{}{}{} ", colors::BLUE, ui::TAG, colors::RESET);
    io::stdout().flush().expect("There is not stdout");

    let mut user_input = String::new();
    scan_input(&mut user_input);

    println!("{}", user_input);

    user_input.to_string()
}
