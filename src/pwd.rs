use std::{env, io};

pub fn pwd() -> Result<(), io::Error> {
    let dir = env::current_dir()?;
    println!("{}", dir.display());
    Ok(())
}
