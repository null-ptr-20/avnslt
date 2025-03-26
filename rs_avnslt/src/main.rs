use std::io;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use avnslt::{Ctrl, scan_input};

fn main() -> io::Result<()> {

    println!("Hello");
    let mut user_input = String::new();
    let decide: &str = scan_input(&mut user_input);
    
    let mut result: Ctrl = match decide {
        "start" => Ctrl::Start,
        "end" => Ctrl::End,
        _ => Ctrl::Start,
    };

    while result == Ctrl::Start {
        execute!(io::stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?
    
        /*
        let mut user_input_in_alt = String::new();
        let choice_in_alt: &str = scan_input(&mut user_input_in_alt);
        
        if choice_in_alt == "end" {
            result = Ctrl::End;
            disable_raw_mode()?;
        }
*/
    }

    execute!(io::stdout(), LeaveAlternateScreen)
}

