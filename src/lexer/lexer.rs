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
    }
}

pub type Token = TokenType;

#[derive(Debug)]
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
    Numeric(String),

    /* for errors */
    Unknown(char),
}

#[derive(Debug)]
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

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str, ) -> Lexer<'a> {
        Lexer {
            cur_col: 1,
            cur_line: 1,

            codepoint_offset: 0,

            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    }

    fn push_balance(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            *v += 1;
            *v
        } else {
            self.balancing_state.insert(*c, 0);
            0
        }
    }

    fn pop_balance(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            if *v == 1 {
                *v -= 1;
                *v
            }
        } else {
            self.balancing_state.insert(*c, 0);
            0
        }
    }

    pub fn transform_to_type(&mut self,c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::Punctuation { raw: c, kind: PunctuationKind::Open(self.push_balance(&c)) }),
            ')' => Some(TokenType::Punctuation { raw: c, kind: PunctuationKind::Close(self.pop_balance(&c))}),
        }
    }
}
