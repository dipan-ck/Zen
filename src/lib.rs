use std::io::{self, Write};

use crate::{
    cd::cd,
    echo::echo,
    external_runner::run_enternal_prog,
    get_type::{BUILTIN_TYPES, get_type},
    pwd::pwd,
};

pub mod cd;
pub mod echo;
pub mod external_runner;
pub mod get_type;
pub mod pwd;

pub fn zen() -> Result<(), io::Error> {
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
            run_enternal_prog(&parts);
        } else {
            // match parts.as_slice() {
            //     ["echo", args @ ..] => echo(&args),
            //     ["type", command] => get_type(command)?,
            //     ["pwd"] => pwd()?,
            //     ["exit"] => return Ok(()),
            //     _ => println!("{}: command not found", parts[0]),
            // }
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
            echo(&args);
            Ok(())
        }
        ["type", command] => {
            get_type(command)?;
            Ok(())
        }
        ["cd", path] => {
            cd(path, current_path)?;
            Ok(())
        }
        ["pwd"] => {
            pwd()?;
            Ok(())
        }
        _ => {
            println!("{}: command not found", parts[0]);
            Ok(())
        }
    }
}
