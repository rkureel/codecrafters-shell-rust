use std::{env, fs, path::PathBuf, process::{Command, Output}};
use cd::Cd;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use typ::Type;
use crate::repl::State;

mod cd;
mod echo;
mod exit;
mod pwd;
mod typ;

#[derive(Debug)]
pub struct ExecutionOutput {
    pub stdout: String,
    pub stderr: String
}

pub trait Execute {
    fn execute(&self, args: &Vec<&str>, state: &mut State) -> ExecutionOutput;
}

pub fn from_str(arg: &str) -> Option<Box<dyn Execute>> {
    match arg {
        "exit" => Some(Box::new(Exit{})),
        "echo" => Some(Box::new(Echo{})),
        "type" => Some(Box::new(Type{})),
        "pwd" => Some(Box::new(Pwd{})),
        "cd" => Some(Box::new(Cd{})),
        _ => None
    }
}

pub fn find_executable_in_path(exec_name: &str) -> Option<PathBuf> {
    let env_path: String = env::var("PATH").unwrap();
    env_path.split(":")
        .into_iter()
        .filter_map(|path_str| fs::read_dir(path_str).ok())
        .flat_map(|entry| {
                entry.map(|e| e.unwrap().path())
        })
        .map(|path_buf| path_buf.to_owned())
        .filter(|path| {
            let file_name: &str = path.file_stem().unwrap().to_str().unwrap();
            file_name.eq(exec_name)
        })
        .next()
}

pub fn execute_executable_in_path(_path: &PathBuf, args: &Vec<&str>) -> ExecutionOutput {
    let arguments: &[&str] = &args[1..];
    let output: Output = Command::new(&args[0])
        .args(arguments)
        .output()
        .unwrap();
    ExecutionOutput {
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap()
    }
}

