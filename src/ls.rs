use std::{
    env, fs,
    io::{self, Write},
};

pub fn ls(writer: &mut dyn Write) -> Result<(), io::Error> {
    let curr_dir = env::current_dir()?;

    let entries = fs::read_dir(curr_dir)?;

    for e in entries {
        writeln!(writer, "{}", e?.file_name().to_string_lossy())?;
    }

    Ok(())
}
