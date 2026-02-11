use rustyline::{
    Context, Helper,
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};

use crate::get_type::BUILTIN_TYPES;

pub struct AutocompleteHelper;

impl Highlighter for AutocompleteHelper {}
impl Helper for AutocompleteHelper {}
impl Validator for AutocompleteHelper {}
impl Hinter for AutocompleteHelper {
    type Hint = String;
}

// Autocompletion works for only Builtin types for now. Suggestion Pair is built from the builtin types array

impl Completer for AutocompleteHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let suggestions: Vec<Pair> = BUILTIN_TYPES
            .iter()
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: format!("{} ", cmd.to_string()),
            })
            .collect();

        let matches: Vec<Pair> = suggestions
            .into_iter()
            .filter(|s| s.display.starts_with(line))
            .collect();

        Ok((0, matches))
    }
}
