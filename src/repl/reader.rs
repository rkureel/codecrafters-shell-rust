use crate::repl::completer::BuiltInCommandCompleter;
use rustyline::{highlight::MatchingBracketHighlighter, hint::HistoryHinter, history::DefaultHistory, validate::MatchingBracketValidator, Config, Editor};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Completer, Hinter, Highlighter, Validator)]
pub struct CustomHelper {
    #[rustyline(Completer)]
    completer: BuiltInCommandCompleter,
    _highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter
}

pub fn new_editor() -> Editor<CustomHelper, DefaultHistory> {
    let config: Config = Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();
    let helper: CustomHelper = CustomHelper {
        completer: BuiltInCommandCompleter::new(),
        _highlighter: MatchingBracketHighlighter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter::new()
    };
    let mut editor = Editor::with_config(config).unwrap();
    editor.set_helper(Some(helper));
    editor
}

