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
        .map(|arg| arg.trim())
        .collect();
    let command: &str = args
        .get(0)
        .unwrap();
    match command {
        "exit" => commands::exit(&args),
        "echo" => commands::echo(&args),
        _ => {
            println!("{}: command not found", command);
            false
        }
    }
}
