use std::{env, fs, path::PathBuf};

pub enum BuiltInCommand {
    Exit,
    Echo,
    Type
}

impl BuiltInCommand {
    pub fn from_str(arg: &str) -> Option<BuiltInCommand> {
        match arg {
            "exit" => Some(BuiltInCommand::Exit),
            "echo" => Some(BuiltInCommand::Echo),
            "type" => Some(BuiltInCommand::Type),
            _ => None
        }
    }

    pub fn run(&self, args: &Vec<&str>) -> bool {
        match self {
            Self::Exit => run_built_in_exit(args),
            Self::Echo => run_built_in_echo(args),
            Self::Type => run_built_in_type(args)
        }
    }
}

fn run_built_in_exit(_args: &Vec<&str>) -> bool {
    true
}

fn run_built_in_echo(args: &Vec<&str>) -> bool {
    let output: String = args[1..]
        .join(" ");
    println!("{}", output);
    false
}

fn run_built_in_type(args: &Vec<&str>) -> bool {
    let arg: &str = args.get(1).unwrap();
    
    if let Some(_) = BuiltInCommand::from_str(arg) {
        println!("{} is a shell builtin", arg);
        return false;
    }

    if let Some(path_buf) = search_executable_path(arg) {
        let exec_path: &str = path_buf.as_path().to_str().unwrap();
        println!("{} is {}", arg, exec_path);
        return false;
    }

    print_cmd_not_found(arg);
    false
}

fn search_executable_path(exec_name: &str) -> Option<PathBuf> {
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


fn print_cmd_not_found(cmd: &str) {
    println!("{}: not found", cmd);
}
