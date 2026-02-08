use std::io;

/*

 It took me a good amount of time and lot of debugging to come up with this tokenization algorithm it may not be perfect or fully efficient but it works but i still decided to add detailed comments throughout the algorithm so that anyone reading this in future can atleast understand my approach and maybe make it even better and those comments will also help me if I forget how i wrote this logic which happens rarely but who knows!!!!

*/

pub fn tokenize(commands: String) -> Result<Vec<String>, io::Error> {
    let commands = commands.trim().as_bytes();
    let mut pos = 0;

    let mut current = Vec::new();
    let mut inside_single_quote = false;
    let mut inside_double_quote = false;
    let mut args = Vec::new();

    while pos < commands.len() {
        match commands[pos] {
            /*
            Inside single quotes no characters has special meaning so we push every byte to the current Vector but we first
            also check that are we already inside a double quote, if yes then the hole logic of single
            quote doesnt apply. the logic of double quotes will be applied
            */
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
            /*
              when we face a double quote first we check if we are already inside a single quote if yes then the rules of single quote will be followed and even the double quote will be pushed to the current vector because as we know in single quote everything preserves but if not inside single quote we set the inside double quote bool to true so that we will process next bytes based on double quotes rules.
            */
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
            /*
              when we face a space  or tab we first check that are we inside a single quote or double quote if yes then we preserve the space by pushing  it to the current vector because as we already know that both single quote and double quote preserves white space inside them. but if not inside any quote we check if current is empty or not if empty we wil do nothing because nothing is there to process but if there is that means we already read a argument because as we know argument are whitespaced ex: echo hello world. so when we space after hello that means the current vector already holds the bytes to convert to string to create the hello string so we push it to the args.
            */
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
            /*
            Now when we face a backslash first we check if we are inside a single quote because inside single quote even backslash doesnt have that escape the next character behaviour its treated as a backspash and pushed the the current vector. but if we are not inside a single quote then we need to ingore the backslash but push the next character to backslash to the vector doesn't mattter what it is we push it and because in this scenario we faced both the backslash and the next character we increment the pos by  2
            */
            b'\\' => {
                if inside_single_quote {
                    current.push(b'\\');
                    pos += 1;
                } else {
                    current.push(commands[pos + 1]);
                    pos += 2;
                }
            }

            /*
            Reaching to this match means this is just a simple character has no special meaning so we wil just process it by pushihg it to the current vector. and incrementing the pos by 1.
            */
            c => {
                current.push(c);
                pos += 1;
            }
        }
    }

    /*
    At last we check the current vector if it's empty or not if yes that means we processed all argumennts and pushed them to the args array if not empty that measn the last  characters we matched and psuhed to current which is technically the last argument so we push it to the args.
    */

    if !current.is_empty() {
        args.push(String::from_utf8(current.clone()).unwrap());
    }

    current.clear();
    Ok(args)
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenize;

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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
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
            let result = tokenize(input.to_string()).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
