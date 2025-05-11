#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    handle_command(input.as_str());
}

fn handle_command(command: &str) {
   println!("{}: command not found", command.trim()); 
}
