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
fn test_register_wrong_command() {
    let s = "   egister    later   ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_no_options() {
    let s = "register";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_later() {
    let s = "register later";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: None, code: None});
}

#[test]
fn test_register_later_with_spaces() {
    let s = "   register    later   ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: None, code: None});
}

#[test]
fn test_register_name_only() {
    let s = "register name Diehard"; 
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: Some("Diehard".to_string()), code: None});
}

#[test]
fn test_register_name_only_with_spaces() {
    let s = "   register   name    King  Queen    "; 
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: Some("King Queen".to_string()), code: None});
}

#[test]
fn test_register_name_only_with_no_name() {
    let s = "   register   name    "; 
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_code_only() {
    let s = "   register   code  ABC  "; 
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: None, code: Some("ABC".to_string())});
}

#[test]
fn test_register_code_only_with_no_code() {
    let s = "   register   code  "; 
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_name_and_code() {
    let s = "   register   name King Queen code abc 123  "; 
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {name: Some("King Queen".to_string()), code: Some("abc 123".to_string())});
}

#[test]
fn test_register_code_and_name() {
    let s = "   register   code King Queen name abc 123  "; 
    let r = Command::parse(s);
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::REGISTER {code: Some("King Queen".to_string()), name: Some("abc 123".to_string())});
}

#[test]
fn test_register_code_and_name_with_empty_name() {
    let s = "   register   code King Queen name  "; 
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_code_and_name_with_empty_code() {
    let s = "   register   code name abc 123  "; 
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_register_code_and_name_with_empty_code_and_name() {
    let s = "   register   code name "; 
    let r = Command::parse(s);
    assert!(r.is_err());
}

