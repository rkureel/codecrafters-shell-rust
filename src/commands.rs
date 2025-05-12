pub fn exit(_args: &Vec<&str>) -> bool {
    true
}

pub fn echo(args: &Vec<&str>) -> bool {
    let output: String = args[1..]
        .join(" ");
    println!("{}", output);
    false
}
