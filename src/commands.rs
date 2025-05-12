pub enum Command {
    Exit,
    Echo,
    Type
}

pub fn get_command_type(arg: &str) -> Option<Command> {
    match arg {
        "exit" => Some(Command::Exit),
        "echo" => Some(Command::Echo),
        "type" => Some(Command::Type),
        _ => None
    }
}

pub fn execute_built_in_command(command: &Command, args: &Vec<&str>) -> bool {
    match command {
        Command::Exit => exec_exit(&args),
        Command::Echo => exec_echo(&args),
        Command::Type => exec_type(&args)
        
    }
}

fn exec_exit(_args: &Vec<&str>) -> bool {
    true
}

fn exec_echo(args: &Vec<&str>) -> bool {
    let output: String = args[1..]
        .join(" ");
    println!("{}", output);
    false
}

fn exec_type(args: &Vec<&str>) -> bool {
    let arg: &str = args.get(1).unwrap();
    match get_command_type(arg) {
        Some(_) => {
            println!("{} is a shell builtin", arg);

            false
        }
        None => {
            println!("{}: not found", arg);
            false
        }
    }
}

