use std::{
    env, fs,
    io::{self, Write},
};

use rustyline::{
    Context, Helper,
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};

use crate::get_type::{BUILTIN_TYPES, is_executable};

pub struct AutocompleteHelper {
    suggestions: Vec<Pair>,
}

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
        let suggestions = &self.suggestions;

        let matches: Vec<Pair> = suggestions
            .iter()
            .filter(|s| s.display.starts_with(line))
            .cloned()
            .collect();

        if matches.is_empty() {
            print!("\x07");
            io::stdout().flush()?;
            return Ok((0, matches));
        }

        if matches.len() == 1 {
            return Ok((0, matches));
        }

        let mut prefix = matches[0].display.clone();

        for m in &matches {
            while !m.display.starts_with(&prefix) {
                prefix.pop();
            }
        }

        if prefix.len() > line.len() {
            return Ok((
                0,
                vec![Pair {
                    display: prefix.clone(),
                    replacement: prefix,
                }],
            ));
        }

        Ok((0, matches))
    }
}

impl AutocompleteHelper {
    pub fn new() -> Self {
        AutocompleteHelper {
            suggestions: build_sugestions().unwrap_or_default(),
        }
    }
}

fn build_sugestions() -> Option<Vec<Pair>> {
    let paths = match env::var_os("PATH") {
        Some(p) => p,
        None => {
            return None;
        }
    };

    let mut suggestions: Vec<Pair> = vec![
        Pair {
            display: "xyz_foo".to_string(),
            replacement: "xyz_foo".to_string(),
        },
        Pair {
            display: "xyz_foo_bar".to_string(),
            replacement: "xyz_foo_bar".to_string(),
        },
        Pair {
            display: "xyz_foo_bar_baz".to_string(),
            replacement: "xyz_foo_bar_baz".to_string(),
        },
    ];

    for dir in env::split_paths(&paths) {
        if let Ok(entries) = fs::read_dir(dir) {
            for e in entries.flatten() {
                if is_executable(&e.path()) {
                    let name = e.file_name().to_string_lossy().into_owned();
                    suggestions.push(Pair {
                        display: name.to_owned(),
                        replacement: name.to_owned(),
                    })
                }
            }
        }
    }

    for cmd in BUILTIN_TYPES {
        suggestions.push(Pair {
            display: cmd.to_string(),
            replacement: cmd.to_string(),
        });
    }

    Some(suggestions)
}
