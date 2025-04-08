pub mod constants;
mod file;

use std::io::{self, Write};
use file::Summary;
use std::ffi::OsString;
use crate::constants::ui;
use crate::constants::colors;

/*
* TODO: create a function to edit file
* read, write, save, add flag
*/
pub fn edit_file(file_name: OsString) {
    todo!();
}

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

    let new_text_file: file::File = file::File::build(title, date, body);

    println!("{}", new_text_file.summarize());

    let _save = new_text_file.save_file();
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
