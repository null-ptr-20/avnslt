pub mod constants;
mod file;

use std::io::{self, Write};
use std::ffi::OsString;
use std::fs;

use file::Summary;
use crate::constants::ui;
use crate::constants::colors;

/*
* Prompt user to create a document
* Fill in the title, date, body
* Show the preview before saving
*/
pub fn prompt_create_file() -> () {
    println!("Please input the fields to start a file: ");

    let title = prompt_scan("Title: ");
    let date = prompt_scan("Date: ");
    let body = loop_body_prompt();
    let access = true;

    let new_text_file: file::CFile = file::CFile::build(title, date, body, access);

    println!("{}", new_text_file.summarize());

    let _save = new_text_file.save_file();
}

/*
* TODO: Open file and put respective fields into a struct
*       add editing logic choices.
*/
pub fn prompt_open_file() {
    let path: OsString = OsString::from(prompt_scan("Please enter the file you want to edit: ").trim());
    let file_contents = fs::read_to_string(path).unwrap();

    println!("{}", file_contents);
}

fn loop_body_prompt() -> Vec<String> {
    let mut body: Vec<String> = Vec::new();
    
    
    loop {
        let line: String = prompt_scan("B_ln: ").trim().to_string();

        if line == ":s" {
            break;
        }

        body.push(line);
    }

    body 
}

/*
* Scan the user input that is entered into the terminal
* remove eol char with trim -> converts into &str
*/
pub fn scan_input(user_input: &mut String) -> &str {
    io::stdin().read_line(user_input).expect("no input");

    user_input.trim()
}

/*
* Prompt user with an output for user to read
* Scan user input in terminal 
* Convert scanned user input into a String
*/
pub fn prompt_scan(output: &str) -> String {
    println!("{}", output);

    print!("{}{}{} ", colors::BLUE, ui::TAG, colors::RESET);
    io::stdout().flush().expect("There is not stdout");

    let mut user_input = String::new();
    scan_input(&mut user_input);

    println!("Your Entry: {}", user_input);

    user_input.to_string()
}
