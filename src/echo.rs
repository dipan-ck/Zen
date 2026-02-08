use std::io::{self, Write};

pub fn echo(args: &[String], writer: &mut dyn Write) -> Result<(), io::Error> {
    writeln!(writer, "{}", args.join(" "))?;

    Ok(())
}
