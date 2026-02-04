use std::io::{self, Write};

use crate::get_type::BUILTIN_TYPES;

pub mod cd;
pub mod echo;
pub mod external_runner;
pub mod get_type;
pub mod pwd;

pub fn run() -> Result<(), io::Error> {
    let mut current_path = String::from("/");

    loop {
        print!("{}$ ", &current_path);
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.len() < 1 {
            continue;
        }

        if parts[0] == "exit" {
            return Ok(());
        }

        if !is_builtin(parts[0].trim()) {
            external_runner::run_enternal_prog(&parts);
        } else {
            run_builtin(&parts, &mut current_path)?;
        }
    }
}

fn is_builtin(command: &str) -> bool {
    if BUILTIN_TYPES.contains(&command) {
        true
    } else {
        false
    }
}

fn run_builtin(parts: &Vec<&str>, current_path: &mut String) -> Result<(), io::Error> {
    match parts.as_slice() {
        ["echo", args @ ..] => {
            echo::echo(&args);
            Ok(())
        }
        ["type", command] => match get_type::get_type(command) {
            Ok(msg) => {
                println!("{}", msg);
                Ok(())
            }
            Err(e) => Err(e),
        },
        ["cd", path] => {
            if let Err(e) = cd::cd(path, current_path) {
                eprintln!("cd: {}", e);
            }
            Ok(())
        }
        ["pwd"] => {
            pwd::pwd()?;
            Ok(())
        }
        _ => {
            println!("{}: command not found", parts[0]);
            Ok(())
        }
    }
}
