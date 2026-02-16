# Zen Shell

A custom Unix shell written in Rust. I built this as a learning project to understand how shells work under the hood. It's not meant to replace your daily shell.

## What It Does

Zen can handle most of the stuff you'd expect from a shell:

- **Builtin commands**: `echo`, `cd`, `pwd`, `ls`, `type`, `history`, `exit`
- **External programs**: Runs anything in your PATH
- **Piping**: Chain commands together with `|`
- **I/O redirection**: Use `>`, `>>`, and `2>` for output redirection
- **Quote handling**: Single quotes, double quotes, and escape characters work as expected
- **Autocompletion**: Tab completion for commands (builtins and executables in PATH)
- **Command history**: Keeps track of your commands and persists them between sessions

## Building

You'll need Rust installed. If you don't have it, grab it from [rustup.rs](https://rustup.rs/).

```bash
cargo build --release
```

The binary will be in `target/release/zen`.

## Running

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/release/zen
```

You'll see a `>> ` prompt where you can start typing commands.

## Examples

Basic commands:
```bash
>> echo hello world
hello world

>> pwd
/home/user/projects/zen

>> ls
main.rs
lib.rs
...
```

Redirection:
```bash
>> echo "some text" > output.txt
>> echo "more text" >> output.txt
>> cat nonexistent 2> errors.txt
```

Piping:
```bash
>> ls | grep ".rs"
>> echo "test" | cat | cat
```

History:
```bash
>> history          # Show all commands
>> history 5        # Show last 5 commands
>> history -w file  # Write history to file
>> history -r file  # Read history from file
```

## How It Works

The shell follows a pretty standard architecture:

1. **Tokenizer** (`tokenizer.rs`): Takes raw input and breaks it into tokens, handling quotes and escape characters
2. **Command Parser** (`command.rs`): Structures tokens into a `Command` struct with program name, arguments, redirections, and pipes
3. **Executor** (`executor.rs`): Runs the command, handling both builtins and external programs
4. **History** (`history.rs`): Manages command history with persistence



## Environment Variables

- `HISTFILE`: Path to history file (defaults to `./history.log`)
- `PATH`: Used to find external commands

## Known Limitations

This is a learning project, so there are some things it doesn't do:

- No job control (background processes, fg/bg)
- No environment variable expansion (like `$HOME`)
- No globbing (wildcards like `*.txt`)
- No command substitution
- Limited signal handling
- Unix/Linux only (uses Unix-specific APIs)

---

## Contributing

Feel free to fork this and play around with it. If you find bugs or want to add features, go for it. This was mainly a learning exercise for me, but I'm happy if it helps anyone else understand how shells work.

## License

Do whatever you want with this code. No restrictions.
