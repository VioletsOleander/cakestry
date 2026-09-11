pub struct CommandParser;

pub enum Command {
    ExitApp,
}

impl CommandParser {
    pub fn parse(command: &str) -> Result<Command, ()> {
        match command {
            "exit" => Ok(Command::ExitApp),
            _ => Err(()),
        }
    }
}
