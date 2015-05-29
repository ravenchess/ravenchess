/**
 * LICENSE
 * 
**/

/** 
 * UCI commands. 
**/

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CommandType {
    UCI,
    DEBUG,
    ISREADY,
    SETOPTION,
    REGISTER,
    UCINEWGAME,
    POSITION,
    GO,
    STOP,
    PONDERHIT,
    QUIT,

    ID,
    UCIOK,
    READYOK,
    BESTMOVE,
    COPYPROTECTION,
    REGISTRATION,
    INFO,
    OPTION,
}

pub enum ParseError { InvalidCommand, }

pub trait Command {
    fn new(cmd: &'static str) -> Result<Self, ParseError>;
    fn get_type(&self) -> CommandType;
}


