use thiserror::Error;

use crate::state::State;

enum Command {
    Exit,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("invalid command")]
    InvalidCommand,
    #[error("invalid argument")]
    InvalidArgument,
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(command: String, state: &mut State) {
        match parse_command(command) {
            Ok(command) => execute_command(command, state),
            Err(err) => state.notification = err.to_string(),
        }
    }
}

fn parse_command(command: String) -> Result<Command, ParseError> {
    match command.as_str() {
        "exit" => Ok(Command::Exit),
        _ => Err(ParseError::InvalidCommand),
    }
}

fn execute_command(command: Command, state: &mut State) {
    match command {
        Command::Exit => state.should_exit = true,
    }
}
