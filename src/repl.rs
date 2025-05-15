use anyhow::Result;

use crate::commands::{self, BuiltInCommand};
use std::{env, io::{self, Write}, path::PathBuf};


pub struct Repl {
    state: State
}

pub struct State {
    pub dir: PathBuf,
    pub continue_repl: bool
}

impl Repl {

    pub fn new() -> Repl {
        Repl {
            state: State{
                dir: env::current_dir().unwrap(),
                continue_repl: true
            }
        }
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            print!("$ ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            self.handle_input(input.as_str())?;
            if !self.state.continue_repl {
                break;
            }
        }
        return Ok(())
    }

    fn handle_input(&mut self, input: &str) -> Result<()> {
        let args: Vec<&str> = input
            .split(" ")
            .map(|arg| arg.trim())
            .collect();

        let command: &str = args
            .get(0)
            .unwrap();

        if let Some(built_in_command) = BuiltInCommand::from_str(command) {
            return built_in_command.run(&args, &mut self.state)
        }

        if let Some(exec_path) = commands::find_command_in_path(command) {
            commands::run_command_in_path(&exec_path, &args);
            return Ok(());
        }

        println!("{}: command not found", command);
        Ok(())
    }

}

