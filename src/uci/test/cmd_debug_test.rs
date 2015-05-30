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
fn test_debug_on() {
    let s = "debug on";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::DEBUG { enabled: true });
}

#[test]
fn test_debug_off() {
    let r = Command::parse("debug off");
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::DEBUG { enabled: false });
}

#[test]
fn test_invalid_command() {
    let r = Command::parse("unexpected");
    assert!(r.is_err());

    let r = Command::parse("invcmd     on");
    assert!(r.is_err());
}

#[test]
fn test_debug_cmd_without_params() {
    let r = Command::parse("debug");
    assert!(r.is_err());
}

#[test]
fn test_debug_on_with_spaces() {
    let s = "   debug      on    ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::DEBUG { enabled: true });
}

#[test]
fn test_debug_off_with_spaces() {
    let s = "   debug      off";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::DEBUG { enabled: false });
}

