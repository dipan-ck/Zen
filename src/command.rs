enum Stream {
    STDOUT,
    STDERR,
    STDIN,
}

enum Mode {
    OVERWRITE,
    APPEND,
}

struct Redirect {
    target: String,
    stream: Stream,
    mode: Mode,
}

pub struct Command {
    pub program: String,
    pub arguments: Vec<String>,
    pub redirects: Vec<Redirect>,
}

impl Redirect {
    fn new(target: String, stream: Stream, mode: Mode) -> Self {
        Redirect {
            target,
            stream,
            mode,
        }
    }
}

impl Command {
    pub fn new(tokens: Vec<String>) -> Self {
        let program = tokens[0].clone();
        let mut arguments = Vec::new();
        let mut redirects = Vec::new();
        let mut pos = 1;

        while pos < tokens.len() {
            match tokens[pos].as_str() {
                ">" if pos + 1 < tokens.len() => {
                    redirects.push(Redirect::new(
                        tokens[pos + 1].clone(),
                        Stream::STDOUT,
                        Mode::OVERWRITE,
                    ));
                    pos += 1;
                }
                ">>" if pos + 1 < tokens.len() => {
                    redirects.push(Redirect::new(
                        tokens[pos + 1].clone(),
                        Stream::STDOUT,
                        Mode::APPEND,
                    ));
                    pos += 1;
                }
                "2>" if pos + 1 < tokens.len() => {
                    redirects.push(Redirect::new(
                        tokens[pos + 1].clone(),
                        Stream::STDERR,
                        Mode::OVERWRITE,
                    ));
                    pos += 1;
                }
                _ => {
                    arguments.push(tokens[0].clone());
                    pos += 1;
                }
            }
        }

        Command {
            program,
            arguments,
            redirects,
        }
    }
}
