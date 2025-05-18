use anyhow::Result;
use crate::commands::{Execute, State};

pub struct Pwd {}

impl Execute for Pwd {
    fn execute(&self, _args: &Vec<String>, state: &mut State) -> Result<()> {
        println!("{}", state.dir.to_str().unwrap());
        return Ok(());
    }
}
