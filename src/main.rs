use std::io::{self, Write};

use commands::{run_executable, search_executable_in_path, BuiltInCommand};

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

    if let Some(built_in_command) = BuiltInCommand::from_str(first_arg) {
        return built_in_command.run(&args)
    }

    if let Some(exec_path) = search_executable_in_path(first_arg) {
        run_executable(&exec_path, &args);
        return false
    }

    println!("{}: command not found", first_arg);
    false
}
