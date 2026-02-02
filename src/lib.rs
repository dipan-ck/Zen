use std::io::{self, Write};

use crate::{echo::echo, get_type::get_type};

pub mod echo;

pub mod get_type;

pub fn zen() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim() == "exit".to_string() {
            break;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["echo", args @ ..] => echo(&args),
            ["type", command] => get_type(command),
            ["exit"] => break,
            [] => {}
            _ => println!("{}: command not found", parts[0]),
        }
    }
}
