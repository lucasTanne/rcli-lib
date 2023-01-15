use std::collections::HashMap;
use crate::command::Command;
use crate::command::Context;
use crate::parser::Parser;

pub struct Rcli {
    name: String,
    commands: HashMap<String, Command>,
}
impl Rcli {
    pub fn new() -> Rcli{
        let default_name = ("myCli").to_string();
        Rcli { name: default_name, commands: HashMap::new() }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_command(&mut self, command: Command) -> Result<(), String> {
        let command_name = command.get_name().to_string();
        let item = self.commands.get(&command_name);

        match item {
            None => {
                self.commands.insert(command_name, command);
                Ok(())
            },
            Some(_) => Err(format!("The command \"{}\" already exist!", command_name))
        }
    }

    pub fn start (&self, args: Vec<String>) -> Result<(), String> {
        let parser: Parser = Parser::new();
        let context = parser.parse(&args);

        match context {
            Err(err) => Err(err),
            Ok(ctx) => {
                let result = self.execute(ctx);

                match result {
                    Err(err) => Err(err),
                    Ok(()) => Ok(())
                }
            }
        }
    }

    fn execute(&self, context: Context) -> Result<(), String> {
        let command_name = context.command_name.to_string();
        let item = self.commands.get(&command_name);

        match item {
            None => Err(format!("The command \"{}\" doesn't exist!", command_name)),
            Some(cmd) => {
                cmd.execute(context);
                Ok(())
            }
        }
    }
}