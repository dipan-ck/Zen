use std::io::{self, Write};

use crate::command::Command;

pub mod cd;
pub mod command;
pub mod echo;
pub mod executor;
pub mod get_type;
pub mod ls;
pub mod pwd;
pub mod tokenizer;

pub fn run() -> Result<(), io::Error> {
    loop {
        print!("$");
        io::stdout().flush().unwrap();

        //read user input  from terminal
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

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
