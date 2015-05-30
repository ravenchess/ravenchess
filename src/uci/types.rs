// Raven is a high performance UCI chess engine
// Copyright (C) 2015-2015 Nam Pham
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Command {
    UCI,
    DEBUG { enabled: bool },
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

pub const UCI_SIG: &'static str = "uci";
pub const DEBUG_SIG: &'static str = "debug";
pub const ISREADY_SIG: &'static str = "isready";

pub enum ParseError { InvalidCommand }

pub trait CommandParser {
    fn parse(line: &str) -> Result<Command, ParseError>;
}

pub trait TokenParser {
    fn parse(tokens: Vec<&str>) -> Result<Command, ParseError>;
}

