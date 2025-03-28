mod colors;
mod ui;
mod file;

use std::io::{self, Write, Result};
use std::fs;
use std::path::Path;
use std::ffi::{OsStr, OsString};


/*
pub fn save(created_file: file::File) -> Result<()> {
    let saved_file_name: AsRef<Path> = OsStr::new("hello.txt");

    let file = String::from(created_file.title);
    
    fs::write(saved_file_name, file)?;
    Ok(())
}
*/

pub fn prompt_create_file() -> file::File {
    println!("Please input the fields to start a file: ");
    let title = prompt_scan("Title: ");
    let date = prompt_scan("Date: ");
    let body = prompt_scan("Body: ");

    let new_text_file: file::File = file::File::build(title, date, body);

    new_text_file
}

pub fn scan_input(user_input: &mut String) -> &str {
    io::stdin().read_line(user_input).expect("no input");

    user_input.trim()
}

/*
pub fn scan_osinput(user_input: &OsStr) -> OsString {
    io::stdin().read_line(user_input).expect("no input");

    user_input.to_os_string()
}
*/

pub fn prompt_scan(output: &str) -> String {
    println!("{}", output);

    print!("{}{}{} ", colors::BLUE, ui::TAG, colors::RESET);
    io::stdout().flush().expect("There is not stdout");

    let mut user_input = String::new();
    scan_input(&mut user_input);

    println!("{}", user_input);

    user_input.to_string()
}

/*
pub fn prompt_scan_os(output: &str) -> OsStr {
    println!("{}", output);

    print!("{}{}{} ", colors::GREEN, ui::TAG, colors::RESET);
    io::stdout().flush().expect("There is not stdout");

    let mut user_input = OsStr::new();
    scan_input(&mut user_input);

    println!("{}", user_input);

    user_input.to_string()
}
*/
