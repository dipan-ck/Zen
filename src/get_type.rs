use std::{env, fs};

pub fn get_type(command: &str) {
    let command = command.trim();
    let built_in_types = ["echo", "exit", "type"];

    if built_in_types.contains(&command) {
        println!("{}: is a shell builtin", command);
    } else {
        let paths = env::var_os("PATH").unwrap();

        let extensions = [
            format!("{command}.exe"),
            format!("{command}.bat"),
            format!("{command}.cmd"),
        ];

        for path in env::split_paths(&paths) {
            for ex in &extensions {
                let path = format!("{}/{}", path.to_str().unwrap(), ex);
                if let Ok(_) = fs::read(&path) {
                    println!("{}", &path);
                    return;
                }
            }
        }

        println!("{} : not found", command);
    }
}
