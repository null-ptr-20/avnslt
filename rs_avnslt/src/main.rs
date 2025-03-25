use std::io;
fn main() {
    println!("Hello, world");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("No user_input");
    user_input.to_string();
    println!("{}", user_input);
}
