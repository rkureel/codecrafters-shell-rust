use crate::commands::{self, Execute, State};
use super::ExecutionOutput;

pub struct Type {}

impl Execute for Type {
    fn execute(&self, args: &Vec<&str>, _state: &mut State) -> ExecutionOutput {
        let arg: &str = args.get(1).unwrap();
        
        if let Some(_) = commands::from_str(arg) {
            return ExecutionOutput {
                stdout: format!("{} is a shell builtin\n", arg),
                stderr: String::new()
            };
        }

        if let Some(path_buf) = commands::find_executable_in_path(arg) {
            let exec_path: &str = path_buf.as_path().to_str().unwrap();
            return ExecutionOutput {
                stdout: format!("{} is {}\n", arg, exec_path),
                stderr: String::new()
            };
        }

        ExecutionOutput {
            stdout: String::new(),
            stderr: format!("{}: not found\n", arg)
        }
    }
}
