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

use super::types::{Command, TokenParser, ParseError, SETOPTION_SIG};

pub enum CmdSetOption {
}

const NAME: &'static str = "name";
const VALUE: &'static str = "value";

impl TokenParser for CmdSetOption {
    fn parse(tokens: Vec<&str>) -> Result<Command, ParseError> {
        match parse_params(tokens) {
            None => Err(ParseError::InvalidCommand),
            Some((name, val)) => Ok(Command::SETOPTION{name: name, value: val})
        }
    }
}

fn parse_params(tokens: Vec<&str>) -> Option<(String, Option<String>)> {
    if tokens.len() < 3 || tokens[0] != SETOPTION_SIG || tokens[1] != NAME {
        return None;
    }

    let mut name: String = tokens[2].to_string();
    let mut val: String = String::new();

    let mut naming: bool = true;

    for i in 3..tokens.len() {
        if naming {
            if tokens[i] != VALUE {
                name = name + " " + tokens[i];
            } else {
                naming = false;
            }
        } else {
            if val.is_empty() {
                val = tokens[i].to_string();
            } else {
                val = val + " " + tokens[i];
            }
        }
    }

    if naming == false && val.is_empty() {
        return None;
    }

    if naming == true {
        return Some((name, None));
    } else {
        return Some((name, Some(val)));
    }
}

