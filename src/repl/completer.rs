use rustyline::{completion::Completer, Context, Result};

pub struct BuiltInCommandCompleter {}

impl BuiltInCommandCompleter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Completer for BuiltInCommandCompleter {
    type Candidate = String;

    fn complete(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<String>)> {
        match line {
            command if "echo".starts_with(command) => {
                return Ok((0, vec![String::from("echo ")]));
            }
            command if "exit".starts_with(command) => {
                return Ok((0, vec![String::from("exit ")]));
            }
            _ => {
                print!("\x07");
                return Ok((line.len()-1, vec![]));
            }
        }
    }
}
