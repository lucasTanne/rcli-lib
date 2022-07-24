# rcli-lib

Rcli-lib is a basic library that offer the possibility to make a command line application easily.

## Why

It's my first rust project to discover this langage and because I love using CLI, I wanted to create my library to make it.

## Import

This library is only available on Github, you cannot found this library in Crate.io.  
So to use this lib in your project you should add the following dependancy in your `Cargo.toml` file:

```toml
[dependencies]
rcli_lib = { git = "https://github.com/lucasTanne/rcli-lib", branch = "master" }
```

Then you can use the `cargo update` command to update your project's dependancies.

## How use rcli

Actually rcli support only command composed by: `command argument`  
The following code is a basic exemple to use rcli:

```rust
use std::env;
use rcli_lib::rcli;
use rcli_lib::command;
use rcli_lib::command::Context;

// Declare a command function
fn print_word(context: Context) {
    let arg = context.argument.clone();
    println!("The magic word is: {:?}", arg.unwrap());
}

fn main() {
    // Gets the executable arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Create a rcli instance
    let mut _rcli = rcli::Rcli::new();

    // Create a new command using the previous function
    let _command = command::Command::new("print", print_word);
    _rcli.add_command(_command);

    // Run rcli to read arguments and run the associated command
    _rcli.start(args);
}
```

Then run it using the following command:

```bash
cargo run print blabla
```

You must have in your terminal: `The magic word is: blabla`.
