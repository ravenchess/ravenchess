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

use super::types::{Command, TokenParser, ParseError, REGISTER_SIG};

pub enum CmdRegister {
}

const LATER: &'static str = "later";
const NAME: &'static str = "name";
const CODE: &'static str = "code";

impl TokenParser for CmdRegister {
    fn parse(tokens: Vec<&str>) -> Result<Command, ParseError> {
        if tokens.len() < 2 || tokens[0] != REGISTER_SIG {
            return Err(ParseError::InvalidCommand);
        }

        if tokens[1] == LATER {
            if tokens.len() == 2 {
                return Ok(Command::REGISTER {name: None, code: None} );
            } else {
                return Err(ParseError::InvalidCommand);
            }
        }

        if tokens[1] != NAME && tokens[1] != CODE {
            return Err(ParseError::InvalidCommand);
        }

        if tokens.len() < 3 {
            return Err(ParseError::InvalidCommand);
        }

        match parse_params(tokens) {
            None => Err(ParseError::InvalidCommand),
            Some((name, code)) => Ok(Command::REGISTER{name: name, code: code})
        }
    }
}

fn parse_params(tokens: Vec<&str>) -> Option<(Option<String>, Option<String>)> {
    let mut naming: bool = tokens[1] == NAME;

    let mut has_name = naming;
    let mut has_code = !naming;

    let mut name: String = String::new();
    let mut code: String = String::new();

    for i in 2..tokens.len() {
        if naming {
            if tokens[i] != CODE {
                if name.is_empty() {
                    name = tokens[i].to_string();
                } else {
                    name = name + " " + tokens[i];
                }
            } else {
                if name.is_empty() {
                    return None;
                }
                naming = false;
                has_code = true;
            }
        } else {
            if tokens[i] != NAME {
                if code.is_empty() {
                    code = tokens[i].to_string();
                } else {
                    code = code + " " + tokens[i];
                }
            } else {
                if code.is_empty() {
                    return None;
                }
                naming = true;
                has_name = true;
            }
        }
    }

    if has_name && name.is_empty() {
        return None;
    }

    if has_code && code.is_empty() {
        return None;
    }

    if has_name && !has_code  {
        return Some((Some(name), None));
    } else if !has_name && has_code {
        return Some((None, Some(code)));
    } else {
        return Some((Some(name), Some(code)));
    }
}

