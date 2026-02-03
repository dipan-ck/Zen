use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs, io};

const BUILTIN_TYPES: [&str; 3] = ["echo", "exit", "type"];

pub fn get_type(command: &str) -> Result<(), io::Error> {
    let command = command.trim();

    if BUILTIN_TYPES.contains(&command) {
        println!("{}: is a shell builtin", command);
    } else {
        let paths = match env::var_os("PATH") {
            Some(p) => p,
            None => {
                println!("{} : not found", command);
                return Ok(());
            }
        };

        for path in env::split_paths(&paths) {
            let candidate = path.join(command);
            if is_executable(&candidate) {
                println!("{}: {}", command, candidate.display());
                return Ok(());
            }
        }

        println!("{} : not found", command);
    }

    Ok(())
}

fn is_executable(path: &Path) -> bool {
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
