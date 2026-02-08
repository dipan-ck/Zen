use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::{self, Stdio},
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
    if !BUILTIN_TYPES.contains(&&command.program.as_str()) {
        run_enternal_prog(&command)?;
        return Ok(());
    }

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

    run_builtin(&command, &mut *stdout, &mut *stderr)?;

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
        "type" => get_type(&command, stdout)?,
        "pwd" => pwd(stdout)?,
        "cd" => cd(&command.arguments)?,
        _ => {
            writeln!(stderr, "{}: command not found", command.program)?;
        }
    }

    Ok(())
}

pub fn run_enternal_prog(command: &Command) -> Result<(), io::Error> {
    let mut cmd = process::Command::new(&command.program);
    cmd.args(&command.arguments);

    let mut file = OpenOptions::new();
    file.create(true).write(true);

    for redirect in &command.redirects {
        match redirect.mode {
            Mode::OVERWRITE => file.truncate(true),
            Mode::APPEND => file.append(true),
        };

        let file = file.open(&redirect.target)?;

        match redirect.stream {
            Stream::STDOUT => cmd.stdout(Stdio::from(file)),
            Stream::STDERR => cmd.stderr(Stdio::from(file)),
            Stream::STDIN => cmd.stdin(Stdio::from(file)), //to be changed later
        };
    }

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
