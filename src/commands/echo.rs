use anyhow::Result;

use crate::commands::{Execute, State};

pub struct Echo {}

impl Execute for Echo {
    fn execute(&self, args: &Vec<String>, _state: &mut State) -> Result<()> {
        let output: String = args[1..]
            .join(" ");
        println!("{}", output);
        Ok(())
    }
}

