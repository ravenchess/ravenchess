/**
**/
use super::types::{Command, CommandType, ParseError};

#[allow(dead_code)]
pub struct CmdUci {
    cmd: &'static str
}

impl Command for CmdUci {

    fn new(cmd: &'static str) -> Result<CmdUci, ParseError> {
        if cmd.starts_with("uci") && cmd.len() == 3 {
            return Ok(CmdUci { cmd: cmd });
        }

        Err(ParseError::InvalidCommand)
    }

    fn get_type(&self) -> CommandType {
        CommandType::UCI
    }
}

