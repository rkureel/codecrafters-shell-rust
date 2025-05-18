use crate::commands::{Execute, State};
use super::ExecutionOutput;

pub struct Exit {}

impl Execute for Exit {
    fn execute(&self, _args: &Vec<&str>, state: &mut State) -> ExecutionOutput {
        state.continue_repl = false;
        ExecutionOutput {
            stdout: String::new(),
            stderr: String::new()
        }
    }
}
