use anyhow::Result;
use crate::{commands::{self, ExecutionOutput}, parser};
use std::{env, fs, io::{self, Write}, path::PathBuf};

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
        let all_args: Vec<String> = parser::parse_input(input);
        let command: &str = all_args
            .get(0)
            .unwrap();

        let command_args: Vec<&str> = all_args
            .iter()
            .take_while(|arg| *arg != ">" && *arg != "1>")
            .map(|s| s.as_str())
            .collect();

        if let Some(built_in_command) = commands::from_str(command) {
            let output: ExecutionOutput = built_in_command.execute(&command_args, &mut self.state);
            Repl::handle_exection_output(&all_args, &output);
            return Ok(());
        }

        if let Some(exec_path) = commands::find_executable_in_path(command) {
            let output: ExecutionOutput = commands::execute_executable_in_path(&exec_path, &command_args);
            Repl::handle_exection_output(&all_args, &output);
            return Ok(());
        }

        println!("{}: command not found", command);
        Ok(())
    }

    fn handle_exection_output(args: &Vec<String>, output: &ExecutionOutput) {
        if !output.stderr.is_empty() {
            print!("{}", &output.stderr);
        }
           
        let mut iterator = args.iter().skip_while(|arg| *arg != ">" && *arg != "1>");
        match iterator.nth(1) {
            Some(path) => {
                fs::write(path, &output.stdout).unwrap();
            }
            None => {
                print!("{}", &output.stdout);
            }
        }
    }
}

