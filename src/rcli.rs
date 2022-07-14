use std::collections::HashMap;
use crate::command;

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

    pub fn execute(&self, command_name: String) {
        let cmd = self.commands.get(&command_name);
        if !cmd.is_none() {
            let command = cmd.unwrap();
            command.execute();
        } else {
            panic!("Command not found");
        }
    }
}