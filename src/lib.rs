use crate::{autocompletion::AutocompleteHelper, command::Command};
use rustyline::{Editor, history::FileHistory};
use std::io::{self};
pub mod autocompletion;
pub mod cd;
pub mod command;
pub mod echo;
pub mod executor;
pub mod get_type;
pub mod ls;
pub mod pwd;
pub mod tokenizer;

pub fn run() -> Result<(), io::Error> {
    let mut rl: Editor<AutocompleteHelper, FileHistory> = Editor::new().unwrap();
    let helper = AutocompleteHelper::new();
    rl.set_helper(Some(helper));

    loop {
        let user_input = rl.readline(">> ");

        let user_input = match user_input {
            Ok(line) => line,
            Err(_) => return Ok(()),
        };

        // optional: save history
        rl.add_history_entry(&user_input).unwrap();

        //tokenize the string input into Vector of String
        let tokens = tokenizer::tokenize(user_input)?;

        //Takes the tokens Vector and returns a Command struct which is passed to an executor
        let command = Command::new(tokens);

        if command.program == "exit" {
            return Ok(());
        }

        // Takes the command struct which holds program, arguments, and redirection and does the execution
        executor::execute(command)?;
    }
}
