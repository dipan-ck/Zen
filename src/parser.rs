use std::io;

pub fn parse(commands: String) -> Result<Vec<String>, io::Error> {
    let commands = commands.trim().as_bytes();
    let mut pos = 0;
    let mut args = Vec::new();

    let mut cmd = Vec::new();

    while pos < commands.len() && commands[pos] != b' ' {
        cmd.push(commands[pos]);
        pos += 1;
    }

    args.push(String::from_utf8(cmd).unwrap());
    //skip the space
    pos += 1;

    while pos < commands.len() {
        //if contains : ' something  '
        if commands[pos] == b'\'' {
            let mut argument = Vec::new();
            pos += 1;
            while pos < commands.len() && commands[pos] != b'\'' {
                argument.push(commands[pos]);
                pos += 1;
            }
            args.push(String::from_utf8(argument).unwrap());
            pos += 1;
        } else {
            if commands[pos] == b' ' {
                pos += 1;
            } else {
                let mut argument = Vec::new();
                while pos < commands.len() && commands[pos] != b' ' {
                    argument.push(commands[pos]);
                    pos += 1;
                }
                args.push(String::from_utf8(argument).unwrap());
            }
        }
    }
    println!("{:?}", args);
    Ok(args)
}
