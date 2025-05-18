use std::{env, path::{Path, PathBuf}};
use anyhow::Result;
use crate::commands::{Execute, State};

pub struct Cd {}

impl Execute for Cd {
    fn execute(&self, args: &Vec<String>, state: &mut State) -> Result<()> {
        let arg: &str = args.get(1).unwrap();
        let resolved_path: std::io::Result<PathBuf> = resolve_path(&mut state.dir, arg);
        match resolved_path {
            Ok(path) => {
                state.dir = path;
            }
            Err(_) => {
                println!("cd: {}: No such file or directory", arg);
            }
        }
        return Ok(());
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
