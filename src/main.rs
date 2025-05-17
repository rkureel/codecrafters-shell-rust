use anyhow::Result;
use repl::Repl;

mod repl;
mod commands;
mod parser;

fn main() -> Result<()> {
    let mut repl: Repl = Repl::new();
    repl.start()?;
    Ok(())
}

