use std::process;
use std::collections::HashMap;
use crate::command;
use crate::command::Context;
use crate::parser;


pub struct Rcli {
    commands: HashMap<String, command::Command>,
}
impl Rcli {
    pub fn new() -> Rcli{
        Rcli { commands: HashMap::new() }
    }

    pub fn add_command(&mut self, command: command::Command) {
        self.commands.insert(command.get_name().to_string(), command);
    }

    fn execute(&self, context: Context) {
        let cmd = self.commands.get(&context.command_name);
        if !cmd.is_none() {
            println!("found: {}", context.command_name);
            let command = cmd.unwrap();
            command.execute(context);
        } else {
            panic!("Command not found");
        }
    }
    pub fn start (&self, args: Vec<String>) {
        let _context = parser::Parser::new(&args).unwrap_or_else(| err | {
            println!("{}", err);
            process::exit(1);
        });

        self.execute(_context);
    }
}