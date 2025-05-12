use std::io::{self, Write};

mod commands;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let exit: bool = handle_input(input.as_str());
        if exit {
            break; 
        }
    }
}

fn handle_input(input: &str) -> bool {
    let args: Vec<&str> = input
        .split(" ")
        .collect();
    let command: &str = args
        .get(0)
        .unwrap()
        .trim();
    match command {
        "exit" => commands::exit(&args),
        _ => {
            println!("{}: command not found", command);
            false
        }
    }
}
