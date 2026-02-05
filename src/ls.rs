use std::{env, fs, io};

pub fn ls() -> Result<(), io::Error> {
    let curr_dir = env::current_dir()?;

    let entries = fs::read_dir(curr_dir)?;

    for e in entries {
        let e = e?.file_name();
        println!("{}", e.to_string_lossy());
    }

    Ok(())
}
