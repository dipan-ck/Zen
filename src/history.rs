use std::{
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

use crate::command::Command;

pub struct History {
    commands: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: &String) {
        self.commands.push(command.to_string());
    }

    pub fn read_history(&self, command: &Command, writer: &mut dyn Write) -> Result<(), io::Error> {
        if command.arguments.is_empty() {
            for (index, command) in self.commands.iter().rev().enumerate() {
                writeln!(writer, "{} {}", index + 1, command)?;
            }
        }

        Ok(())
    }
}

pub static HISTORY: LazyLock<Mutex<History>> = LazyLock::new(|| Mutex::new(History::new()));
