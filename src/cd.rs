use std::{env, io, path::Path};

pub fn cd(path: &str) -> Result<(), io::Error> {
    let path = Path::new(path);
    env::set_current_dir(path)?;
    Ok(())
}
