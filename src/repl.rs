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
        let args: Vec<String> = Repl::parse_input(input);
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

    fn parse_input(input: &str) -> Vec<String> {
        enum Mode {
            Normal,
            SingleQuotes
        }

        let mut args: Vec<String> = Vec::new();
        let mut mode: Mode = Mode::Normal;
        let mut current_word: Vec<char> = Vec::new();

        for ch in input.chars() {
            match ch {
                '\n' => {
                    if !current_word.is_empty() {
                        args.push(current_word.into_iter().collect());
                    }
                    break;
                }
                '\'' => {
                    match mode {
                        Mode::Normal => {
                            mode = Mode::SingleQuotes;
                        }
                        Mode::SingleQuotes => {
                            mode = Mode::Normal;
                        }
                    }
                }
                ' ' => {
                    match mode {
                        Mode::Normal => {
                            if !current_word.is_empty() {
                                args.push(current_word.iter().collect()); 
                                current_word.clear();
                            }
                        }
                        Mode::SingleQuotes => {
                            current_word.push(ch);
                        }
                    }
                }
                other => {
                    current_word.push(other);
                }
            }
        }

        args
    }
}

