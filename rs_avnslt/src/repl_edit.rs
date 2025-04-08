/*
* NOTE: This is to create a simple module in the main file in order to:
* main {REPLedit::start}
*/

pub mod repl_editor {
    use avnslt::{prompt_scan, prompt_create_file};

    pub fn run() {
        eprintln!("Starting avnslt repl editor...");
        let input = prompt_scan("Choose an option:\n1. Create File\n2. Edit File");

        match input.trim().parse::<u8>() {
            Ok(user_input) => {
                match user_input {
                    1 => prompt_create_file(),
                    _ => println!("Invalid choice!"),
                }
            }
            Err(error) => {
                println!("Error! @type -> {}", error);
            }
        }
    }
    pub fn quit() {
        todo!()
    }
}
