use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{io::Write, path::PathBuf};

use crate::command::Command;
use std::{env, fs, io};

pub const BUILTIN_TYPES: [&str; 7] = ["echo", "history", "ls", "exit", "type", "pwd", "cd"];

pub fn get_type(command: &Command, writer: &mut dyn Write) -> Result<(), io::Error> {
    if BUILTIN_TYPES.contains(&&command.arguments[0].as_str()) {
        writeln!(writer, "{}: is a shell builtin", command.arguments[0])?;
        return Ok(());
    };

    match search_external(command) {
        Some((_, candidate)) => {
            writeln!(writer, "{}: {}", command.arguments[0], candidate)?;
        }
        None => {
            writeln!(writer, "{}: not found", command.arguments[0])?;
        }
    };

    Ok(())
}

pub fn is_executable(path: &Path) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !metadata.is_file() {
        return false;
    }

    let mode = metadata.permissions().mode();
    mode & 0o111 != 0
}

fn search_external(command: &Command) -> Option<(String, String)> {
    let paths = match env::var_os("PATH") {
        Some(p) => p,
        None => {
            return None;
        }
    };

    let mut candidate: PathBuf;

    for path in env::split_paths(&paths) {
        candidate = path.join(&command.arguments[0]);
        if is_executable(&candidate) {
            return Some((
                command.arguments[0].clone(),
                candidate.to_string_lossy().into_owned(),
            ));
        }
    }

    None
}
