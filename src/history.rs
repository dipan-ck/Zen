use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    sync::{LazyLock, Mutex},
};

use crate::command::Command;

pub struct History {
    commands: Vec<String>,
    history_file_entries: usize,
}

impl History {
    pub fn new() -> Self {
        let hist_file_path = env::var("HISTFILE").unwrap_or_else(|_| String::from("./history.log"));
        println!("History fetched from : {}", hist_file_path);
        let contents = fs::read_to_string(hist_file_path).unwrap_or_default();
        let commands: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
        let history_file_entries = commands.len();
        Self {
            commands,
            history_file_entries,
        }
    }

    pub fn add(&mut self, command: &String) {
        self.commands.push(command.to_owned());
    }

    pub fn persist_history(&self) -> Result<(), io::Error> {
        let mut options = OpenOptions::new();
        options.create(true).append(true).write(true);

        let mut file = options.open("./history.log")?;

        for c in &self.commands[self.history_file_entries..] {
            writeln!(file, "{}", c)?;
        }

        Ok(())
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
                let history_file_content = fs::read_to_string(path)?;

                let commands: Vec<String> =
                    history_file_content.lines().map(|x| x.to_owned()).collect();

                self.commands.extend_from_slice(&commands[..]);
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
