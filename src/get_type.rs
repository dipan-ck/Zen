use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs, io};

use crate::command::Command;

pub const BUILTIN_TYPES: [&str; 6] = ["echo", "ls", "exit", "type", "pwd", "cd"];

pub fn get_type(command: &Command, writer: &mut dyn Write) -> Result<(), io::Error> {
    if BUILTIN_TYPES.contains(&&command.arguments[0].as_str()) {
        writeln!(writer, "{}: is a shell builtin", command.arguments[0])?;
        return Ok(());
    };

    let paths = match env::var_os("PATH") {
        Some(p) => p,
        None => {
            writeln!(writer, "{}: not found", command.arguments[0])?;
            return Ok(());
        }
    };

    for path in env::split_paths(&paths) {
        let candidate = path.join(&command.arguments[0]);
        if is_executable(&candidate) {
            writeln!(writer, "{}: {}", command.arguments[0], candidate.display())?;
            return Ok(());
        }
    }
    // not found
    writeln!(writer, "{}: not found", command.arguments[0])?;

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

// #[cfg(test)]
// mod tests {
//     use crate::get_type::get_type;

//     #[test]
//     fn builtin_test() {
//         let msg = get_type("echo").unwrap();
//         assert_eq!(msg, "echo: is a shell builtin");
//     }

//     #[test]
//     fn external_program() {
//         let msg = get_type("cat").unwrap();

//         // must start with "cat: "
//         assert!(msg.starts_with("cat: "));

//         // must contain an absolute path
//         let path = msg.strip_prefix("cat: ").unwrap();
//         assert!(path.starts_with('/'));
//     }

//     #[test]
//     fn not_found_program_test() {
//         let msg = get_type("invalid_program").unwrap();

//         //will return "invalid_program : not found"
//         assert_eq!(msg, "invalid_program : not found");
//     }
// }
