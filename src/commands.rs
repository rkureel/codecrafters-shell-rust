use std::{env, fs, path::{Path, PathBuf}, process::Command};
use anyhow::Result;
use crate::repl::State;

pub enum BuiltInCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd
}

impl BuiltInCommand {
    pub fn from_str(arg: &str) -> Option<BuiltInCommand> {
        match arg {
            "exit" => Some(BuiltInCommand::Exit),
            "echo" => Some(BuiltInCommand::Echo),
            "type" => Some(BuiltInCommand::Type),
            "pwd" => Some(BuiltInCommand::Pwd),
            "cd" => Some(BuiltInCommand::Cd),
            _ => None
        }
    }

    pub fn run(&self, args: &Vec<&str>, state: &mut State) -> Result<()> {
        match self {
            Self::Exit => run_built_in_exit(args, state),
            Self::Echo => run_built_in_echo(args, state),
            Self::Type => run_built_in_type(args, state),
            Self::Pwd => run_built_in_pwd(args, state),
            Self::Cd => run_built_in_cd(args, state)
        }
    }
}

pub fn find_command_in_path(exec_name: &str) -> Option<PathBuf> {
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

pub fn run_command_in_path(_path: &PathBuf, args: &Vec<&str>) {
    let arguments: &[&str] = &args[1..];
    Command::new(args[0])
        .args(arguments)
        .status()
        .unwrap();
}

fn run_built_in_exit(_args: &Vec<&str>, state: &mut State) -> Result<()> {
    state.continue_repl = false;
    Ok(())
}

fn run_built_in_echo(args: &Vec<&str>, _state: &mut State) -> Result<()> {
    let output: String = args[1..]
        .join(" ");
    println!("{}", output);
    Ok(())
}

fn run_built_in_type(args: &Vec<&str>, _state: &mut State) -> Result<()> {
    let arg: &str = args.get(1).unwrap();
    
    if let Some(_) = BuiltInCommand::from_str(arg) {
        println!("{} is a shell builtin", arg);
        return Ok(());
    }

    if let Some(path_buf) = find_command_in_path(arg) {
        let exec_path: &str = path_buf.as_path().to_str().unwrap();
        println!("{} is {}", arg, exec_path);
        return Ok(());
    }

    print_cmd_not_found(arg);
    return Ok(());
}

fn run_built_in_pwd(_args: &Vec<&str>, state: &mut State) -> Result<()> {
    println!("{}", state.dir.to_str().unwrap());
    return Ok(());
}

fn run_built_in_cd(args: &Vec<&str>, state: &mut State) -> Result<()> {
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

fn print_cmd_not_found(cmd: &str) {
    println!("{}: not found", cmd);
}

