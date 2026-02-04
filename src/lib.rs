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
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.len() < 1 {
            continue;
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
            run_built_in(&parts)?;
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

fn run_built_in(parts: &Vec<&str>) -> Result<(), io::Error> {
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
            cd(path)?;
            Ok(())
        }
        ["pwd"] => {
            pwd()?;
            Ok(())
        }
        ["exit"] => return Ok(()),
        _ => {
            println!("{}: command not found", parts[0]);
            Ok(())
        }
    }
}
