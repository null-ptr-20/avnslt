use std::io;

#[derive(PartialEq)]
pub enum Ctrl {
    Start,
    End,
}

pub fn scan_input(user_input:&mut String) -> &str {
    io::stdin().read_line(user_input).expect("There is no user input");   
    println!("{}", user_input); 

    user_input
}


