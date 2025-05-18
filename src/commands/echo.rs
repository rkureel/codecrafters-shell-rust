use crate::commands::{Execute, State};
use super::ExecutionOutput;

pub struct Echo {}

impl Execute for Echo {
    fn execute(&self, args: &Vec<&str>, _state: &mut State) -> ExecutionOutput {
        
        let output_str: String = args[1..]
            .join(" ");

        ExecutionOutput {
            stdout: format!("{}\n", output_str),
            stderr: String::new()
        }
    }
}

