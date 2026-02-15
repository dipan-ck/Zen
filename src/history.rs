use std::{
    fs::{self, OpenOptions},
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

    pub fn read_history(
        &mut self,
        command: &Command,
        writer: &mut dyn Write,
    ) -> Result<(), io::Error> {
        match &command.arguments[..] {
            [] => {
                if command.arguments.is_empty() {
                    for (index, command) in self.commands.iter().enumerate() {
                        writeln!(writer, "{} {}", index + 1, command)?;
                    }
                }
            }
            [max] => {
                let max = max.parse::<usize>().unwrap();
                let start = self.commands.len().saturating_sub(max);

                for (index, command) in self.commands.iter().enumerate().skip(start) {
                    writeln!(writer, "{} {}", index + 1, command)?;
                }
            }
            [flag, path] if flag == "-r" => {
                let mut command = Vec::new();
                let mut current = Vec::new();

                let history_file_content = fs::read(path)?;

                let mut pos = 0;

                while pos < history_file_content.len() {
                    match history_file_content[pos] {
                        b'\n' => {
                            if !&current.is_empty() {
                                command.push(String::from_utf8(current.to_owned()).unwrap());
                                current.clear();
                                pos += 1;
                            }
                        }
                        _ => {
                            current.push(history_file_content[pos]);
                            pos += 1;
                        }
                    }
                }

                self.commands.extend_from_slice(&command[..]);
            }
            [flag, path] if flag == "-w" => {
                let mut options = OpenOptions::new();

                options.read(true).write(true).append(true).create(true);
                let mut file = options.open(path)?;

                for command in &self.commands {
                    writeln!(file, "{}", command)?;
                }
            }
            _ => {
                writeln!(writer, "Invalid arguments")?;
            }
        }

        Ok(())
    }
}

pub static HISTORY: LazyLock<Mutex<History>> = LazyLock::new(|| Mutex::new(History::new()));
