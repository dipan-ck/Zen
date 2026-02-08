use std::{
    env,
    io::{self, Write},
};

pub fn pwd(writer: &mut dyn Write) -> Result<(), io::Error> {
    let dir = env::current_dir()?;
    writeln!(writer, "{}", dir.display())?;
    Ok(())
}
