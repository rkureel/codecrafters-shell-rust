use anyhow::Result;
use repl::Repl;

mod repl;
mod commands;

fn main() -> Result<()> {
    let mut repl: Repl = Repl::new();
    repl.start()?;
    Ok(())
}

