use crate::command::Context;

pub struct Parser {}
impl Parser {
    pub fn new() -> Parser {
        Parser {  }
    }

    pub fn parse(&self, args: &[String]) -> Result<Context, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        
        let command_name: String;
        let argument: Option<String>;
        if args.len() == 2 {
            command_name = "helper".to_string();
            argument = None;
        } else {
            command_name = args[1].clone();
            argument = Some(args[2].clone());
        }
            
        Ok(Context { command_name, argument })
    }
}