use std::{
    fs::OpenOptions,
    io::{self, Write},
};

use crate::{
    cd::cd,
    command::{Command, Mode, Redirect, Stream},
    echo::echo,
    get_type::{BUILTIN_TYPES, get_type},
    ls::ls,
    pwd::pwd,
};

pub fn execute(command: Command) -> Result<(), io::Error> {
    let mut stdout: Box<dyn Write> = Box::new(io::stdout());
    let mut stderr: Box<dyn Write> = Box::new(io::stderr());

    for redirect in &command.redirects {
        match redirect.stream {
            Stream::STDOUT => {
                stdout = build_writer(redirect)?;
            }
            Stream::STDERR => {
                stderr = build_writer(redirect)?;
            }
            _ => {}
        };
    }

    if BUILTIN_TYPES.contains(&command.program.as_str()) {
        run_builtin(&command, &mut *stdout, &mut *stderr)?;
    }

    Ok(())
}

fn build_writer(redirect: &Redirect) -> Result<Box<dyn Write>, io::Error> {
    let mut file_open_options = OpenOptions::new();

    file_open_options.write(true).create(true);

    match redirect.mode {
        Mode::APPEND => {
            file_open_options.append(true);
        }
        Mode::OVERWRITE => {
            file_open_options.truncate(true);
        }
    };

    let file = file_open_options.open(&redirect.target)?;

    Ok(Box::new(file))
}

fn run_builtin(
    command: &Command,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> Result<(), io::Error> {
    match command.program.as_str() {
        "echo" => echo(&command.arguments, stdout)?,
        "ls" => ls(stdout)?,
        "type" => get_type(&command.arguments, stdout)?,
        "pwd" => pwd(stdout)?,
        "cd" => cd(&command.arguments)?,
        _ => {
            writeln!(stderr, "{}: command not found", command.program)?;
        }
    }

    Ok(())
}
