use std::{env, process};
use command::Context;

mod rcli;
mod command;
mod parser;

// TEST
fn bla(context: Context) {
    println!("Le mot magique est: {}", context.argument.unwrap());
}

fn main() {
    println!("Welcom to rcli !");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let _query = parser::Parser::new(&args).unwrap_or_else(| err | {
        println!("{}", err);
        process::exit(1);
    });

    let cmd = command::Command::new("bla", bla);

    let mut _rcli = rcli::Rcli::new();
    _rcli.add_command(cmd);

    _rcli.start(args);
}