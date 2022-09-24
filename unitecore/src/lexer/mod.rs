pub mod lexer;
pub mod macros;

pub use lexer::*;
pub use macros::*;

use std::io; 

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("Was expecting {expected:?}, found {found:?}")]
    MissingExpectedSymbol {
        expected: TokenType,
        found: Token
    },

    #[error("Can't create numeric literal due to invalid character {raw:?}")]
    NumericLiteralInvalidChar {
        raw: String,
    },

    #[error("Can't find opening symbol for {symbol:?}")]
    MisbalancedSymbol {
        symbol: char,
        open: char
    },

    #[error("Unkwon Symbol")]
    UnknownSymbol {
        symbol: String,
    }
}

pub type Token = TokenType;

// pub struct Punctuation {
//     pub raw: char,
//     pub kind: PunctuationKind
// }

#[derive(Debug, PartialEq)]
pub enum NumericHint {
    Integer,
    FloatingPoint,
}


#[derive(Debug, PartialEq)]
pub enum TokenType {
    /* End of token Stream */
    EOF,

    /* Punctuation like , ( [ */
    Punctuation{raw: char, kind: PunctuationKind},

    /* Operators e. g. '*', '->' */
    Operators(String),

    /* A sequence of characters */
    Identifier(String),

    /* A single character 'a' => unicode codepoint (32bit). */
    Char(char),

    /* algebraic data type */
    // Numeric{raw: String, base: NumericBaseKind, postfix: NumberPostfixKind, form: NumericForm},
    Numeric{ raw: String, hint: NumericHint},

    /* for errors */
    Unknown(char),
}

#[derive(Debug, PartialEq)]
pub enum PunctuationKind {
    /* '(', '[', '{' */
    Open(BalancingDepthType),
    /* ')', ']', '}' */
    Close(BalancingDepthType),
    /* ',', ';' */
    Separator
}

type BalancingDepthType = i32;

pub struct Lexer<'a> {
    /* human readable */
    pub cur_line: usize,
    pub cur_col: usize,
    /* raw format in 'codepoints' */
    pub codepoint_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,
    balancing_state: std::collections::HashMap<char, i32>,
}
