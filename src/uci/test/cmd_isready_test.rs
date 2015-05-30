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

use super::super::types::*;

#[test]
fn test_create_good_isready_command() {
    let r = Command::parse("isready");
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::ISREADY);
}

#[test]
fn test_create_good_isready_command_with_spaces() {
    let r = Command::parse("     isready    ");
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd == Command::ISREADY);
}


#[test]
fn test_create_bad_isready_command() {
    let r = Command::parse("nonexistence");
    assert!(r.is_err());
}

#[test]
fn test_create_good_isready_command_but_with_extra_params() {
    let r = Command::parse("isready param1 param2");
    assert!(r.is_err());
}

