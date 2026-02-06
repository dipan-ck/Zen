use std::io;

pub fn parse(commands: String) -> Result<Vec<String>, io::Error> {
    let commands = commands.trim().as_bytes();
    let mut pos = 0;

    let mut current = Vec::new();
    let mut inside_single_quote = false;
    let mut args = Vec::new();

    while pos < commands.len() {
        match commands[pos] {
            b'\'' => {
                inside_single_quote = true;
                pos += 1;
                continue;
            }

            b' ' => {
                if !current.is_empty() {
                    args.push(String::from_utf8(current.clone()).unwrap());
                    current.clear();
                }
                pos += 1;
            }

            c => {
                current.push(c);
                pos += 1;
            }
        }
    }

    args.push(String::from_utf8(current.clone()).unwrap());
    current.clear();

    println!("{:?}", args);
    Ok(args)
}
