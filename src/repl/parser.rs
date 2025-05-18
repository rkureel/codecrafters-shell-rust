enum Mode {
    Normal,
    SingleQuotes,
    DoubleQuotes,
    Escape,
    Preserve
}

pub fn parse_input(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut mode: Mode = Mode::Normal;
    let mut current_word: Vec<char> = Vec::new();

    for ch in input.chars() {
        match mode {
            Mode::Normal => {
                match ch {
                    '\n' | ' ' => {
                        append_word_to_args(&mut current_word, &mut args);
                    }
                    '\'' => {
                        mode = Mode::SingleQuotes;
                    }
                    '"' => {
                        mode = Mode::DoubleQuotes;
                    }
                    '\\' => {
                        mode = Mode::Preserve;
                    }
                    _ => {
                        current_word.push(ch);
                    }
                }
            }
            Mode::SingleQuotes => {
                match ch {
                    '\'' => {
                        mode = Mode::Normal;
                    }
                    _ => {
                        current_word.push(ch);
                    }
                }
            }
            Mode::DoubleQuotes => {
                match ch {
                    '"' => {
                        mode = Mode::Normal;
                    }
                    '\\' => {
                        mode = Mode::Escape;
                        current_word.push(ch);
                    }
                    _ => {
                        current_word.push(ch);
                    }
                }
            }
            Mode::Escape => {
                match ch {
                    '\n' | '$' | '`' | '"' | '\\' => {
                        current_word.pop();
                    }
                    _ => ()
                }
                mode = Mode::DoubleQuotes;
                current_word.push(ch);

            }
            Mode::Preserve => {
                mode = Mode::Normal;
                current_word.push(ch);
            }
        };
    }

    if !current_word.is_empty() {
        append_word_to_args(&mut current_word, &mut args); 
    }

    args
}

fn append_word_to_args(word: &mut Vec<char>, args: &mut Vec<String>) {
    if !word.is_empty() {
        args.push(word.iter().collect());
        word.clear();
    }
}
