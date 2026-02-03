use std::io::{self, Write};

use crate::{
    echo::echo,
    external_runner::run_enternal_prog,
    get_type::{BUILTIN_TYPES, get_type},
};

pub mod echo;
pub mod external_runner;
pub mod get_type;

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
            match parts.as_slice() {
                ["echo", args @ ..] => echo(&args),
                ["type", command] => get_type(command)?,
                ["exit"] => return Ok(()),
                [] => {}
                _ => println!("{}: command not found", parts[0]),
            }
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
