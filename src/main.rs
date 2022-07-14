mod rcli;
mod command;

use std::env;

// TEST
fn bla(word: String) {
    println!("Le mot magique est: {}", word);
}

fn main() {
    println!("Welcom to rcli !");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let cmd: command::Command = command::Command::new("bla", bla);

    let mut _rcli = rcli::Rcli::new();
    _rcli.add_command(cmd);
}