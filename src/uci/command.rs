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

use super::types::*;

use super::cmd_uci::CmdUci;
use super::cmd_debug::CmdDebug;
use super::cmd_isready::CmdIsReady;
use super::cmd_setoption::CmdSetOption;
use super::cmd_register::CmdRegister;

impl Command {
    // Normalize and tokenize a string
    fn tokenize(cmd: &str) -> Vec<&str> {
        cmd.split(' ').filter(|s| s.trim().len() > 0).collect()
    }
}

impl CommandParser for Command {
    fn parse(line: &str) -> Result<Self, ParseError> {
        let tokens = Command::tokenize(line);

        if tokens.len() == 0 {
            return Err(ParseError::InvalidCommand);
        }

        match tokens[0] {
            UCI_SIG => CmdUci::parse(tokens),
            DEBUG_SIG => CmdDebug::parse(tokens),
            ISREADY_SIG => CmdIsReady::parse(tokens),
            SETOPTION_SIG => CmdSetOption::parse(tokens),
            REGISTER_SIG => CmdRegister::parse(tokens),
            _ => return Err(ParseError::InvalidCommand),
        }
    }
}

