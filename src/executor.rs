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

use os_pipe::pipe;
use std::io::Read;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::{self, Stdio},
};

use crate::history::HISTORY;
use crate::{
    cd::cd,
    command::{Command, Mode, Stream},
    echo::echo,
    get_type::{BUILTIN_TYPES, get_type},
    ls::ls,
    pwd::pwd,
};

pub fn run_builtin(
    command: &Command,
    _stdin: Option<Box<dyn Read>>,
    stdout: Option<Box<dyn Write>>,
) -> Result<(), io::Error> {
    let mut stdout = stdout.unwrap_or(Box::new(io::stdout()));
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

    match command.program.as_str() {
        "echo" => echo(&command.arguments, &mut stdout)?,
        "ls" => ls(&mut stdout)?,
        "type" => get_type(&command, &mut stdout)?,
        "pwd" => pwd(&mut stdout)?,
        "cd" => cd(&command.arguments)?,
        "history" => {
            let history = HISTORY.lock().unwrap();

            history.read_history(command, &mut stdout)?;
        }
        _ => {
            writeln!(stderr, "{}: command not found", command.program)?;
        }
    }

    Ok(())
}

fn run_external(
    command: &Command,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
) -> Result<process::Child, io::Error> {
    let mut program = process::Command::new(&command.program);
    program.args(&command.arguments);

    if let Some(s) = stdout {
        program.stdout(s);
    };

    if let Some(s) = stdin {
        program.stdin(s);
    };

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

    program.spawn()
}

pub fn run_pipeline(command: &Command) -> Result<(), io::Error> {
    let mut stdin: Option<Stdio> = None;
    let mut processes = Vec::new();
    let mut current = Some(command);

    while let Some(cmd) = current {
        let needs_pipe = cmd.piped_command.is_some();

        let stdout = if needs_pipe {
            Some(Stdio::piped())
        } else {
            None
        };

        if BUILTIN_TYPES.contains(&cmd.program.as_str()) {
            // Decide where builtin writes

            let writer: Option<Box<dyn Write>>;

            if needs_pipe {
                // create pipe
                let (reader, pipe_writer) = pipe()?;

                // builtin writes into pipe
                writer = Some(Box::new(pipe_writer));

                // next stage reads from pipe
                stdin = Some(Stdio::from(reader));
            } else {
                // last stage → terminal
                writer = Some(Box::new(io::stdout()));
            }

            // run builtin
            run_builtin(
                cmd, None, // stdin later
                writer,
            )?;

            current = cmd.piped_command.as_deref();
            continue;
        }

        let mut process = run_external(cmd, stdin.take(), stdout)?;

        if needs_pipe {
            stdin = Some(Stdio::from(process.stdout.take().unwrap()));
        }

        processes.push(process);
        current = cmd.piped_command.as_deref();
    }

    for mut p in processes {
        p.wait()?;
    }

    Ok(())
}
