pub struct Context {
    pub command_name: String,
    pub argument: Option<String>,
}
type Action = fn(context: Context);

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

    pub fn execute(&self, context: Context) {
        (self.action)(context);
    }
}