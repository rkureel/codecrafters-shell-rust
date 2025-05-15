use std::env;

use repl::{Repl, State};

mod repl;
mod commands;

fn main() {
    let mut state: State = State {
        dir: env::current_dir().unwrap()
    };

    let mut repl: Repl = Repl::new(state);
    repl.start();
}

