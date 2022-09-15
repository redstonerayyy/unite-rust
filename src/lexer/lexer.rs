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

    fn map_balance(c: &char) -> char {
        match c {
            '}' => '{',
            '{' => '}',
            ']' => '[',
            '[' => ']',
            ')' => '(',
            '(' => ')',
            _ => panic!("balancing panic")
        }
    }

    fn push_symbol(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            *v += 1;
            *v - 1
        } else {
            self.balancing_state.insert(*c, 1);
            0
        }
    }

    fn pop_symbol(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        if let Some(v) = self.balancing_state.get_mut(&Lexer::map_balance(&c)) {
            if *v >= 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::MisbalancedSymbol{ symbol: *c, open: Lexer::map_balance(&c)})
            }
        } else {
            Err(LexerError::MisbalancedSymbol{ symbol: *c, open: Lexer::map_balance(&c)})
        }
    }

    fn parse_number(&mut self, start: char) -> Result<TokenType, LexerError> {
        
    }

    fn transform_to_type(&mut self,c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Open(self.push_symbol(&c)) }),
            ')' | ']' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?)}),
            '0' ..= '9' => self.parse_number(c),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() })
        }
    }

    fn consume_char(&mut self ) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }
                self.codepoint_offset += 1;

                return Some(c);
            },
            None => None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume_char();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();

        if let Some(c) = self.consume_char() {
            self.transform_to_type(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
}
