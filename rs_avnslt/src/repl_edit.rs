/*
* NOTE: This is to create a simple module in the main file in order to:
* main {REPLedit::start}
*/

pub mod repl_editor {
    use avnslt::{prompt_scan, prompt_create_file, prompt_open_file};

    pub fn run() {
        println!("");
        println!("Starting avnslt repl editor...\n");
        let input = prompt_scan("Choose an option:\n
            1. Create File
            2. Open File
            3. Edit File\n");

        match input.trim().parse::<u8>() {
            Ok(user_input) => {
                match user_input {
                    1 => prompt_create_file(),
                    2 => prompt_open_file(),
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
