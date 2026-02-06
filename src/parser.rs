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
                    inside_single_quote = !inside_single_quote;
                    pos += 1;
                    continue;
                } else {
                    current.push(b'\'');
                    pos += 1;
                }
            }

            b'"' => {
                if !inside_single_quote {
                    inside_double_quote = !inside_double_quote;
                    pos += 1;
                    continue;
                } else {
                    current.push(b'"');
                    pos += 1;
                }
            }

            b' ' | b'\t' => {
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
    if !current.is_empty() {
        args.push(String::from_utf8(current.clone()).unwrap());
    }

    current.clear();
    Ok(args)
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_single_quote_behaviour() {
        let test_cases = vec![
            // Spaces are preserved within quotes
            ("echo 'hello    world'", vec!["echo", "hello    world"]),
            // Consecutive spaces are collapsed unless quoted
            ("echo hello    world", vec!["echo", "hello", "world"]),
            // Adjacent quoted strings are concatenated
            ("echo 'hello''world'", vec!["echo", "helloworld"]),
            // Empty quotes are ignored
            ("echo hello''world", vec!["echo", "helloworld"]),
            ("echo 'shell hello'", vec!["echo", "shell hello"]),
            ("echo 'world     test'", vec!["echo", "world     test"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_double_quote_behaviour() {
        let test_cases = vec![
            // Multiple spaces preserved
            ("echo \"hello    world\"", vec!["echo", "hello    world"]),
            // Quoted strings next to each other are concatenated
            ("echo \"hello\"\"world\"", vec!["echo", "helloworld"]),
            // Separate arguments
            ("echo \"hello\" \"world\"", vec!["echo", "hello", "world"]),
            // Single quotes inside are literal
            ("echo \"shell's test\"", vec!["echo", "shell's test"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_backslash_outside_quotes() {
        let test_cases = vec![
            // Each backslash-space creates a literal space as part of one argument
            ("echo three\\ \\ \\ spaces", vec!["echo", "three   spaces"]),
            // Backslash preserves first space, subsequent unescaped spaces are collapsed
            ("echo before\\     after", vec!["echo", "before ", "after"]),
            // \n becomes just n
            ("echo test\\nexample", vec!["echo", "testnexample"]),
            // First backslash escapes second, result is single literal backslash
            ("echo hello\\\\world", vec!["echo", "hello\\world"]),
            // Backslash makes single quotes literal
            ("echo \\'hello\\'", vec!["echo", "'hello'"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_backslash_in_single_quotes() {
        let test_cases = vec![
            // Backslashes have no special meaning in single quotes
            (
                "echo 'shell\\\\\\nscript'",
                vec!["echo", "shell\\\\\\nscript"],
            ),
            ("echo 'example\\\"test'", vec!["echo", "example\\\"test"]),
            (
                "echo 'multiple\\\\slashes'",
                vec!["echo", "multiple\\\\slashes"],
            ),
            (
                "echo 'every\\\"thing_is\\\"literal'",
                vec!["echo", "every\\\"thing_is\\\"literal"],
            ),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_backslash_in_double_quotes() {
        let test_cases = vec![
            // \\ escapes backslash
            (
                "echo \"A \\\\ escapes itself\"",
                vec!["echo", "A \\ escapes itself"],
            ),
            // \" escapes double quote
            (
                "echo \"A \\\" inside double quotes\"",
                vec!["echo", "A \" inside double quotes"],
            ),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_quoted_executable_names() {
        let test_cases = vec![
            // Single quoted executable
            ("'my program' argument1", vec!["my program", "argument1"]),
            // Double quoted executable
            (
                "\"exe with spaces\" file.txt",
                vec!["exe with spaces", "file.txt"],
            ),
            // Mixed quotes in executable
            ("\"my 'program'\" arg", vec!["my 'program'", "arg"]),
            ("'my \"program\"' arg", vec!["my \"program\"", "arg"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_mixed_quoting() {
        let test_cases = vec![
            // Mix of single and double quotes
            ("echo 'hello'\"world\"", vec!["echo", "helloworld"]),
            ("echo \"hello\"'world'", vec!["echo", "helloworld"]),
            // Quotes with unquoted parts
            ("echo hello'world'test", vec!["echo", "helloworldtest"]),
            ("echo hello\"world\"test", vec!["echo", "helloworldtest"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_whitespace_handling() {
        let test_cases = vec![
            // Leading/trailing spaces
            ("  echo  arg  ", vec!["echo", "arg"]),
            // Multiple spaces between args
            ("echo    arg1    arg2", vec!["echo", "arg1", "arg2"]),
            // Tabs and spaces
            ("echo\t\targ", vec!["echo", "arg"]),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_complex_real_world_cases() {
        let test_cases = vec![
            (
                "git commit -m \"Fix bug #123\"",
                vec!["git", "commit", "-m", "Fix bug #123"],
            ),
            (
                "ls -la '/path with spaces'",
                vec!["ls", "-la", "/path with spaces"],
            ),
            ("echo \"Don't stop\"", vec!["echo", "Don't stop"]),
            (
                "grep 'pattern' \"file name.txt\"",
                vec!["grep", "pattern", "file name.txt"],
            ),
        ];

        for (input, expected) in test_cases {
            let result = parse(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
