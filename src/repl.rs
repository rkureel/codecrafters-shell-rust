use anyhow::Result;
use reader::CustomHelper;
use rustyline::{error::ReadlineError, history::DefaultHistory, Editor};
use crate::commands::{self, ExecutionOutput};
use std::{env, fs::{File, OpenOptions}, io::Write, path::PathBuf};


mod parser;
mod reader;
mod completer;

pub struct Repl {
    state: State,
    reader: Editor<CustomHelper, DefaultHistory> 
}

pub struct State {
    pub dir: PathBuf,
    pub continue_repl: bool
}

const STDOUT_REDIRECTION_SYMBOLS: [&str;2] = [">", "1>"];
const STDERR_REDIRECTION_SYMBOLS: [&str;1] = ["2>"];
const STDOUT_APPEND_SYMBOLS: [&str;2] = [">>", "1>>"];
const STDERR_APPEND_SYMBOLS: [&str;1] = ["2>>"];

impl Repl {

    pub fn new() -> Repl {
        
        Repl {
            state: State{
                dir: env::current_dir().unwrap(),
                continue_repl: true
            },
            reader: reader::new_editor()

        }
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            let input: rustyline::Result<String> = self.reader.readline("$ ");
            match input {
                Ok(line) => {
                    self.handle_input(line.as_str())?;
                    if !self.state.continue_repl {
                        break; 
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    break
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
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
            .take_while(|arg| !Repl::is_redirection_symbol(arg))
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
        let mut stderr_iter = args.iter().skip_while(|arg| !STDERR_REDIRECTION_SYMBOLS.contains(&arg.as_str())
            && !STDERR_APPEND_SYMBOLS.contains(&arg.as_str()));
        match stderr_iter.next() {
            Some(symbol) => {
                let path: &str = stderr_iter.next().unwrap().as_str();
                let append: bool = STDERR_APPEND_SYMBOLS.contains(&symbol.as_str());
                let mut file: File = OpenOptions::new()
                   .write(true)
                   .append(append)
                   .create(true)
                   .open(path)
                   .unwrap();
                file.write(&output.stderr.as_bytes()).unwrap();
            }
            None => {
                print!("{}", &output.stderr);
            }
        }

        let mut stdout_iter = args.iter().skip_while(|arg| !STDOUT_REDIRECTION_SYMBOLS.contains(&arg.as_str())
            && !STDOUT_APPEND_SYMBOLS.contains(&arg.as_str()));
        match stdout_iter.next() {
            Some(symbol) => {
                let path: &str = stdout_iter.next().unwrap().as_str();
                let append: bool = STDOUT_APPEND_SYMBOLS.contains(&symbol.as_str());
                let mut file: File = OpenOptions::new()
                    .write(true)
                    .append(append)
                    .create(true)
                    .open(path)
                    .unwrap();
                file.write(&output.stdout.as_bytes()).unwrap();
            }
            None => {
                print!("{}", &output.stdout);
            }
        }
    }

    fn is_redirection_symbol(arg: &str) -> bool {
        STDOUT_REDIRECTION_SYMBOLS.contains(&arg)
            || STDERR_REDIRECTION_SYMBOLS.contains(&arg)
            || STDOUT_APPEND_SYMBOLS.contains(&arg)
            || STDERR_APPEND_SYMBOLS.contains(&arg)
    }
}

