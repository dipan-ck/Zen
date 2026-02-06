use std::io;

pub fn parse(commands: String) -> Result<Vec<String>, io::Error> {
    let commands = commands.trim().as_bytes();
    let mut pos = 0;

    let mut current = Vec::new();
    let mut inside_single_quote = false;
    let mut inside_double_quote = false;
    let mut args = Vec::new();

    while pos < commands.len() {
        match commands[pos] {
            b'\'' => {
                if !inside_double_quote {
                    inside_single_quote = true;
                    pos += 1;
                    continue;
                } else {
                    current.push(b'\'');
                    pos += 1;
                }
            }

            b'"' => {
                inside_double_quote = true;
                pos += 1;
                continue;
            }

            b' ' => {
                if inside_single_quote || inside_double_quote {
                    current.push(b' ');
                } else {
                    if !current.is_empty() {
                        args.push(String::from_utf8(current.clone()).unwrap());
                        current.clear();
                    }
                }

                pos += 1;
            }

            b'\\' => {
                if inside_single_quote {
                    current.push(b'\\');
                    pos += 1;
                } else {
                    current.push(commands[pos + 1]);
                    pos += 2;
                }
            }

            c => {
                current.push(c);
                pos += 1;
            }
        }
    }

    args.push(String::from_utf8(current.clone()).unwrap());
    current.clear();

    Ok(args)
}
