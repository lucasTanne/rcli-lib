type Action = fn(a: String);

pub struct Command {
    name: &'static str,
    action: Action,
}
impl Command {
    pub fn new(name: &'static str, action: Action) -> Command {
        Command { name: name, action }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn execute(&self) {
        (self.action);
    }
}