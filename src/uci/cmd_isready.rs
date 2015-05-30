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

use super::types::{Command, TokenParser, ParseError, ISREADY_SIG};

pub enum CmdIsReady {
}

impl TokenParser for CmdIsReady {
    fn parse(tokens: Vec<&str>) -> Result<Command, ParseError> {
        if tokens.len() != 1 || tokens[0] != ISREADY_SIG {
            return Err(ParseError::InvalidCommand);
        }
        Ok(Command::ISREADY)
    }
}

