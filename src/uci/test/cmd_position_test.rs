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
fn test_position_standard_command() {
    let s = "position fen rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 moves e2e4 e7e5";
    let r = Command::parse(s);
    assert!(r.is_ok());

    match r.ok().unwrap() {
        Command::POSITION{fen: f, moves: m} => {
            assert!(f.connect(" ") == "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
            assert!(m.connect(" ") == "e2e4 e7e5");
        },
        _ => {},
    }
}

#[test]
fn test_position_standard_command_with_spaces() {
    let s = "  position   fen   rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R   b   KQkq - 1 2 moves e2e4 e7e5   ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    match r.ok().unwrap() {
        Command::POSITION{fen: f, moves: m} => {
            assert!(f.connect(" ") == "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
            assert!(m.connect(" ") == "e2e4 e7e5");
        },
        _ => {},
    }
}

#[test]
fn test_position_startpos_command() {
    let s = "  position   startpos moves e2e4 e7e5   ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    match r.ok().unwrap() {
        Command::POSITION{fen: f, moves: m} => {
            assert!(f.connect(" ") == "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            assert!(m.connect(" ") == "e2e4 e7e5");
        },
        _ => {},
    }
}

#[test]
fn test_position_startpos_command_without_moves() {
    let s = "  position   startpos ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    match r.ok().unwrap() {
        Command::POSITION{fen: f, moves: m} => {
            assert!(f.connect(" ") == "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            assert!(m.connect(" ") == "");
        },
        _ => {},
    }
}

#[test]
fn test_position_standard_command_without_moves() {
    let s = "position fen rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2 ";
    let r = Command::parse(s);
    assert!(r.is_ok());

    match r.ok().unwrap() {
        Command::POSITION{fen: f, moves: m} => {
            assert!(f.connect(" ") == "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
            assert!(m.connect(" ") == "");
        },
        _ => {},
    }
}

#[test]
fn test_position_only() {
    let s = "position  ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_position_and_fen_without_fen_data() {
    let s = "position fen ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_position_and_fen_without_fen_data_followed_by_moves() {
    let s = "position fen moves";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_position_and_fen_without_fen_data_followed_by_moves_and_move_data() {
    let s = "position fen moves e1e2 e3e4";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_position_and_moves_without_fen_data() {
    let s = "position moves e2e3 ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

#[test]
fn test_position_and_moves_without_moves_data() {
    let s = "position fen abc moves  ";
    let r = Command::parse(s);
    assert!(r.is_err());
}

