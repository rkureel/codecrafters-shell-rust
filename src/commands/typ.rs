use anyhow::Result;

use crate::commands::{self, Execute, State};

pub struct Type {}

impl Execute for Type {
    fn execute(&self, args: &Vec<String>, _state: &mut State) -> Result<()> {
        let arg: &str = args.get(1).unwrap();
        
        if let Some(_) = commands::from_str(arg) {
            println!("{} is a shell builtin", arg);
            return Ok(());
        }

        if let Some(path_buf) = commands::find_executable_in_path(arg) {
            let exec_path: &str = path_buf.as_path().to_str().unwrap();
            println!("{} is {}", arg, exec_path);
            return Ok(());
        }

        println!("{}: not found", arg);
        return Ok(());
    }
}
