#[derive(Debug)]
pub enum Stream {
    STDOUT,
    STDERR,
    STDIN,
}
#[derive(Debug)]
pub enum Mode {
    OVERWRITE,
    APPEND,
}
#[derive(Debug)]
pub struct Redirect {
    pub target: String,
    pub stream: Stream,
    pub mode: Mode,
}

#[derive(Debug)]
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
                    pos += 2;
                }
                ">>" if pos + 1 < tokens.len() => {
                    redirects.push(Redirect::new(
                        tokens[pos + 1].clone(),
                        Stream::STDOUT,
                        Mode::APPEND,
                    ));
                    pos += 2;
                }
                "2>" if pos + 1 < tokens.len() => {
                    redirects.push(Redirect::new(
                        tokens[pos + 1].clone(),
                        Stream::STDERR,
                        Mode::OVERWRITE,
                    ));
                    pos += 2;
                }
                _ => {
                    arguments.push(tokens[pos].clone());
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
