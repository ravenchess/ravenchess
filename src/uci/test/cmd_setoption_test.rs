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

use super::super::types::{Command, CommandParser};

#[test]
fn test_setoption_name_only() {
    let s = "setoption name option1";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::SETOPTION {name: "option1".to_string(), value: None});
}

#[test]
fn test_setoption_name_only_with_spaces_in_name() {
    let s = "setoption name option u1";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::SETOPTION {name: "option u1".to_string(), value: None});
}

#[test]
fn test_setoption_name_only_with_spaces() {
    let s = "    setoption    name   option   u1  ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::SETOPTION {name: "option u1".to_string(), value: None});
}

#[test]
fn test_setoption_with_name_and_value() {
    let s = "setoption name option value v1";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::SETOPTION {name: "option".to_string(), value: Some("v1".to_string())});
}

#[test]
fn test_setoption_with_name_and_value_with_spaces() {
    let s = "   setoption    name    option v2    value   v1  v2    ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::SETOPTION {name: "option v2".to_string(), value: Some("v1 v2".to_string())});
}

#[test]
fn test_setoption_with_name_and_empty_value() {
    let s = "   setoption    name    option v2    value   ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_setoption_no_name_param() {
    let s = "    setoption    asb   option1  ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

