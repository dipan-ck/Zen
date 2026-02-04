use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs, io};

pub const BUILTIN_TYPES: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

pub fn get_type(command: &str) -> Result<String, io::Error> {
    let command = command.trim();

    if BUILTIN_TYPES.contains(&command) {
        return Ok(format!("{}: is a shell builtin", command));
    };

    let paths = match env::var_os("PATH") {
        Some(p) => p,
        None => return Ok(format!("{} : not found", command)),
    };

    for path in env::split_paths(&paths) {
        let candidate = path.join(command);
        if is_executable(&candidate) {
            return Ok(format!("{}: {}", command, candidate.display()));
        }
    }

    Ok(format!("{} : not found", command))
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

#[cfg(test)]
mod tests {
    use crate::get_type::get_type;

    #[test]
    fn builtin_test() {
        let msg = get_type("echo").unwrap();
        assert_eq!(msg, "echo: is a shell builtin");
    }

    #[test]
    fn external_program() {
        let msg = get_type("cat").unwrap();

        // must start with "cat: "
        assert!(msg.starts_with("cat: "));

        // must contain an absolute path
        let path = msg.strip_prefix("cat: ").unwrap();
        assert!(path.starts_with('/'));
    }

    #[test]
    fn not_found_program_test() {
        let msg = get_type("invalid_program").unwrap();

        //will return "invalid_program : not found"
        assert_eq!(msg, "invalid_program : not found");
    }
}
