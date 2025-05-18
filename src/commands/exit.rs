use anyhow::Result;
use crate::commands::{Execute, State};

pub struct Exit {}

impl Execute for Exit {
    fn execute(&self, _args: &Vec<String>, state: &mut State) -> Result<()> {
        state.continue_repl = false;
        Ok(())
    }
}
