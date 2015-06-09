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

// This parser is a Finite State Machine.

// State definition:
//   * IN => Initial state
//   * F0 => Expecting FE token or SP
//   * F1 => Expecting the first GE token
//   * F2 => Expecting another GE token or MV or EO
//   * M0 => Expecting MV or EO token
//   * M1 => Expecting the first MV
//   * M2 => Expecting another move or EO
//   * ER => Final state, the FSM is in error
//   * FN => Final state, no error

// Token definition:
//   * PS = "position"
//   * SP = "startpos"
//   * FE = "fen"
//   * MV = "moves"
//   * GE = general token
//   * EO = End of line

//                 The state transition table
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// |    | IN | F0 | F1 | F2 | M0 | M1 | M2 | <= CURRENT STATE |
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// | PS | F0 | ER | ER | ER | ER | ER | ER |                  |
// | SP | ER | M0 | ER | ER | ER | ER | ER |                  |
// | FE | ER | F1 | ER | ER | ER | ER | ER | <= NEXT STATE    |
// | MV | ER | ER | ER | M1 | M1 | ER | ER |                  |
// | GE | ER | ER | F2 | F2 | ER | M2 | M2 |                  |
// | EO | ER | ER | ER | FN | FN | ER | FN |                  |
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

use super::types::{Command, TokenParser, ParseError, POSITION_SIG, FSM};
use chess::types::Move;

pub enum CmdPosition {
}

const STARTPOS: &'static str = "startpos";
const FEN: &'static str = "fen";
const MOVES: &'static str = "moves";

#[derive(PartialEq)]
enum State {
    IN, F0, F1, F2, M0, M1, M2, ER, FN,
}

enum Token {
    PS,
    SP,
    FE,
    MV,
    GE(String),
    EO,
}

struct FSMHandler {
    pub fen: Vec<String>,
    pub moves: Vec<String>,
}

impl FSMHandler {
    fn push_fen(&mut self, s: String) {
        self.fen.push(s);
    }

    fn push_moves(&mut self, s: String) {
        self.moves.push(s);
    }

    fn push_starting_fen(&mut self) {
        self.push_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
        self.push_fen("w".to_string());
        self.push_fen("KQkq".to_string());
        self.push_fen("-".to_string());
        self.push_fen("0".to_string());
        self.push_fen("1".to_string());
    }
}

impl FSM<Token, FSMHandler> for State {
    fn consume(&self, token: &Token, handler: &mut FSMHandler) -> State {
        match *self {
            State::IN =>
                match *token {
                    Token::PS => State::F0,
                    _  => State::ER,
                },

            State::F0 =>
                match *token {
                    Token::SP => {
                        handler.push_starting_fen();
                        State::M0
                    },
                    Token::FE => return State::F1,
                    _ => State::ER,
                },

            State::F1 =>
                match *token {
                    Token::GE(ref f) => {
                        handler.push_fen(f.to_string());
                        State::F2
                    },
                    _ => State::ER,
                },

            State::F2 =>
                match *token {
                    Token::MV => State::M1,
                    Token::GE(ref f) => {
                        handler.push_fen(f.to_string());
                        State::F2
                    },
                    Token::EO => State::FN,
                    _ => State::ER,
                },

            State::M0 =>
                match *token {
                    Token::MV => State::M1,
                    Token::EO => State::FN,
                    _ => State::ER,
                },

            State::M1 =>
                match *token {
                    Token::GE(ref m) => {
                        handler.push_moves(m.to_string());
                        State::M2
                    },
                    _ => State::ER,
                },

            State::M2 =>
                match *token {
                    Token::GE(ref m) => {
                        handler.push_moves(m.to_string());
                        State::M2
                    },
                    Token::EO => State::FN,
                    _ => State::ER,
                },

            _ => State::ER,
        }
    }

    fn next_token(tokens: &Vec<&str>, idx: &mut usize) -> Token {
        if *idx >= tokens.len() - 1 {
            return Token::EO;
        }

        *idx += 1;
        match tokens[*idx] {
            POSITION_SIG => Token::PS,
            STARTPOS => Token::SP,
            FEN => Token::FE,
            MOVES => Token::MV,
            _ => Token::GE(tokens[*idx].to_string()),
        }
    }

    fn is_final(&self) -> bool {
        *self == State::FN || *self == State::ER
    }

    fn is_err(&self) -> bool {
        *self == State::ER
    }

}

impl TokenParser for CmdPosition {
    fn parse(tokens: Vec<&str>) -> Result<Command, ParseError> {
        if tokens[0] != POSITION_SIG {
            return Err(ParseError::InvalidCommand);
        }

        let mut state = State::IN;
        let mut token = Token::PS;

        let mut idx: usize = 0;
        let mut handler = FSMHandler{ fen: vec![], moves: vec![] };

        while !state.is_final() {
            state = state.consume(&token, &mut handler);
            token = State::next_token(&tokens, &mut idx);
        }

        match state {
            State::ER => Err(ParseError::InvalidCommand),
            State::FN => Ok(Command::POSITION{fen: handler.fen, moves: handler.moves}),
            _ => Err(ParseError::InvalidCommand),
        }
    }
}

