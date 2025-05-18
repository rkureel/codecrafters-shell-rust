use crate::commands::{Execute, State};
use super::ExecutionOutput;

pub struct Pwd {}

impl Execute for Pwd {
    fn execute(&self, _args: &Vec<&str>, state: &mut State) -> ExecutionOutput {
        ExecutionOutput {
            stdout: format!("{}\n", state.dir.to_str().unwrap()),
            stderr: String::new()
        }
    }
}
