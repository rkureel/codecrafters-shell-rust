enum Mode {
    Normal,
    SingleQuotes,
    DoubleQuotes
}

pub fn parse_input(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut mode: Mode = Mode::Normal;
    let mut current_word: Vec<char> = Vec::new();

    for ch in input.chars() {
        match ch {
            '\n' => {
                if !current_word.is_empty() {
                    args.push(current_word.into_iter().collect());
                }
                break;
            }
            '\'' => {
                match mode {
                    Mode::Normal => {
                        mode = Mode::SingleQuotes;
                    }
                    Mode::SingleQuotes => {
                        mode = Mode::Normal;
                    }
                    Mode::DoubleQuotes => {
                        current_word.push(ch);
                    }
                }
            }
            '"' => {
                match mode {
                    Mode::Normal => {
                        mode = Mode::DoubleQuotes;
                    }
                    Mode::DoubleQuotes => {
                        mode = Mode::Normal;
                    }
                    Mode::SingleQuotes => ()
                }
            }
            ' ' => {
                match mode {
                    Mode::Normal => {
                        if !current_word.is_empty() {
                            args.push(current_word.iter().collect()); 
                            current_word.clear();
                        }
                    }
                    _ => {
                        current_word.push(ch);
                    }
                }
            }
            other => {
                current_word.push(other);
            }
        }
    }

    args
}

