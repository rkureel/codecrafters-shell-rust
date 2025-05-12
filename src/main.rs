use std::io::{self, Write};

use commands::Command;

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
    let first_arg: &str = args
        .get(0)
        .unwrap();

    let command_opt: Option<Command> = commands::get_command_type(first_arg);

    match command_opt {
        Some(command) => commands::execute_built_in_command(&command, &args),
        None => {
            println!("{}: command not found", first_arg);
            false
        }
    }
}
