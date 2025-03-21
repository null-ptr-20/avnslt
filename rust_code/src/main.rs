
fn main() {
    let select = Greeting::Hello;

    let choice = match select {
        Greeting::Hello => println!("HELLO"),
        Greeting::Bye => println!("BYE"),
        _ => println!("invalid"),
    };

    choice
}

 enum Greeting {
    Hello,
    Bye
}
