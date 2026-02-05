use std::io::{self, Write};

use crate::get_type::BUILTIN_TYPES;

pub mod cd;
pub mod echo;
pub mod external_runner;
pub mod get_type;
pub mod ls;
pub mod parser;
pub mod pwd;
pub fn run() -> Result<(), io::Error> {
    let mut current_path = String::from("/");

    loop {
        print!("{}$ ", &current_path);
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parsed_commands: Vec<String> = parser::parse(command)?;

        if parsed_commands.is_empty() {
            continue;
        }

        if parsed_commands[0] == "exit" {
            return Ok(());
        }

        if !is_builtin(parsed_commands[0].trim()) {
            external_runner::run_enternal_prog(&parsed_commands);
        } else {
            run_builtin(&parsed_commands, &mut current_path)?;
        }
    }
}

fn is_builtin(command: &str) -> bool {
    BUILTIN_TYPES.contains(&command)
}

fn run_builtin(parsed_commands: &Vec<String>, current_path: &mut String) -> Result<(), io::Error> {
    match parsed_commands.as_slice() {
        [cmd, args @ ..] if cmd == "echo" => {
            echo::echo(args);
            Ok(())
        }
        [cmd, command] if cmd == "type" => match get_type::get_type(command) {
            Ok(msg) => {
                println!("{}", msg);
                Ok(())
            }
            Err(e) => Err(e),
        },
        [cmd, path] if cmd == "cd" => {
            if let Err(e) = cd::cd(path, current_path) {
                eprintln!("cd: {}", e);
            }
            Ok(())
        }
        [cmd] if cmd == "pwd" => {
            pwd::pwd()?;
            Ok(())
        }
        [cmd] if cmd == "ls" => {
            ls::ls()?;
            Ok(())
        }
        _ => {
            println!("{}: command not found", parsed_commands[0]);
            Ok(())
        }
    }
}
