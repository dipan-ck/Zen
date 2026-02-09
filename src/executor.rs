/*
The executor takes a command struct which contains the program, arguments and array of redirections, Now all shells resolve
redirections from left to right and this one is no exeption. When the program is a external type we call the external program
runner which spawns a process and creates file for each redirection and then sets the last file created from the last redirection as the stdout or stderr foir the process to run.

For builtin type we create a stdout and stderr which intitially holds the defaults but when looping through the redirection we assign
values to them that has a Write trait. because the stdout we create will be sent as a s arguments to the bultin programs which prints results
using writeln! macro and this macro takes a value that implements the Write trait.

****FOR BETTER UNDERSTANDING OF EXECUTOR I SUGGEST YOU TO FIRST GO THROUGH THE COMMAND MODULE,
TO UNDERSTAND HOW WE ARE STRUCTURING THE TOKENS INTO STRUCT.****
*/

use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::{self, Stdio},
};

use crate::{
    cd::cd,
    command::{Command, Mode, Stream},
    echo::echo,
    get_type::{BUILTIN_TYPES, get_type},
    ls::ls,
    pwd::pwd,
};

pub fn execute(command: Command) -> Result<(), io::Error> {
    if !BUILTIN_TYPES.contains(&&command.program.as_str()) {
        run_external_program(&command)?
    }

    let mut stdout: Box<dyn Write> = Box::new(io::stdout());
    let mut stderr: Box<dyn Write> = Box::new(io::stderr());

    for redirect in &command.redirects {
        let mut option = OpenOptions::new();
        option.create(true).write(true);

        match redirect.stream {
            Stream::STDOUT => {
                match redirect.mode {
                    Mode::APPEND => option.append(true),
                    Mode::OVERWRITE => option.truncate(true),
                };

                let file = option.open(&redirect.target)?;

                stdout = Box::new(file);
            }
            Stream::STDERR => {
                match redirect.mode {
                    Mode::APPEND => option.append(true),
                    Mode::OVERWRITE => option.truncate(true),
                };

                let file = option.open(&redirect.target)?;

                stderr = Box::new(file)
            }
            Stream::STDIN => {}
        };
    }

    run_builtin(&command, &mut stdout, &mut stderr);

    Ok(())
}
fn run_builtin(
    command: &Command,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> Result<(), io::Error> {
    match command.program.as_str() {
        "echo" => echo(&command.arguments, stdout)?,
        "ls" => ls(stdout)?,
        "type" => get_type(&command, stdout)?,
        "pwd" => pwd(stdout)?,
        "cd" => cd(&command.arguments)?,
        _ => {
            writeln!(stderr, "{}: command not found", command.program)?;
        }
    }

    Ok(())
}

fn run_external_program(command: &Command) -> Result<(), io::Error> {
    let mut program = process::Command::new(&command.program);
    program.args(&command.arguments);

    for redirect in &command.redirects {
        let mut option = OpenOptions::new();
        option.create(true).write(true);

        match redirect.stream {
            Stream::STDOUT => match redirect.mode {
                Mode::APPEND => {
                    option.append(true);
                    let file = option.open(&redirect.target)?;
                    program.stdout(Stdio::from(file));
                }
                Mode::OVERWRITE => {
                    option.truncate(true);
                    let file = option.open(&redirect.target)?;
                    program.stdout(Stdio::from(file));
                }
            },

            Stream::STDERR => match redirect.mode {
                Mode::APPEND => {
                    option.append(true);
                    let file = option.open(&redirect.target)?;
                    program.stderr(Stdio::from(file));
                }
                Mode::OVERWRITE => {
                    option.truncate(true);
                    let file = option.open(&redirect.target)?;
                    program.stderr(Stdio::from(file));
                }
            },

            Stream::STDIN => {}
        }
    }

    program.spawn()?.wait()?;

    Ok(())
}
