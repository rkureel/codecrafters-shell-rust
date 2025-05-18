use std::{env, path::{Path, PathBuf}};
use crate::commands::{Execute, State};

use super::ExecutionOutput;

pub struct Cd {}

impl Execute for Cd {
    fn execute(&self, args: &Vec<&str>, state: &mut State) -> ExecutionOutput {
        let arg: &str = args.get(1).unwrap();
        let resolved_path: std::io::Result<PathBuf> = resolve_path(&mut state.dir, arg);
        return match resolved_path {
            Ok(path) => {
                state.dir = path;
                ExecutionOutput{
                    stdout: String::new(),
                    stderr: String::new()
                }
            }
            Err(_) => {
                ExecutionOutput {
                    stdout: String::new(),
                    stderr: format!("cd: {}: No such file or directory\n", arg)
                }
            }
        }
    }
}

fn resolve_path(current_dir_path: &mut PathBuf, arg: &str) -> std::io::Result<PathBuf> {
    let mut arg_path: PathBuf = PathBuf::from(arg);
    if arg_path.starts_with("~") {
        let home_dir_path: PathBuf = PathBuf::from(env::var("HOME").unwrap());
        let stripped_path: &Path = arg_path.strip_prefix("~").unwrap();
        arg_path = home_dir_path.join(stripped_path);
    }
    let new_path = current_dir_path.join(arg_path);
    new_path.canonicalize()
}
