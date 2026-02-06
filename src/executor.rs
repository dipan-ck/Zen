use std::io;

use crate::{cd, command::Command, echo, get_type, ls, pwd};

pub fn execute(command: Command) -> Result<(), io::Error> {
    Ok(())
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
