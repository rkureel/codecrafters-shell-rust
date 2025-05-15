use crate::commands::{self, BuiltInCommand};
use std::{io::{self, Write}, path::PathBuf};


pub struct Repl {
    state: State
}

pub struct State {
    pub dir: PathBuf
}

impl Repl {

    pub fn new(state: State) -> Repl {
        Repl {state}
    }

    pub fn start(&self) {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let exit: bool = self.handle_input(input.as_str());
            if exit {
                break; 
            }
        }
    }

    fn handle_input(&self, input: &str) -> bool {
        let args: Vec<&str> = input
            .split(" ")
            .map(|arg| arg.trim())
            .collect();

        let command: &str = args
            .get(0)
            .unwrap();

        if let Some(built_in_command) = BuiltInCommand::from_str(command) {
            return built_in_command.run(&args, &self.state)
        }

        if let Some(exec_path) = commands::find_command_in_path(command) {
            commands::run_command_in_path(&exec_path, &args);
            return false
        }

        println!("{}: command not found", command);
        false
    }

}

