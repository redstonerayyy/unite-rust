use std::io;


#[derive(Error, Debug)]
pub enum LexerError {
    #[error("")]
    FileIO(#[from] io::Error),

    MissingExpectedSymbol {
        expected: ??,
        found: ??
    }
}


pub type Token = TokenType;

pub enum TokenType {
    /* End of token Stream */
    EOF,

    /* Punctuation like , ( [ */
    Punctuation{raw: char, kind:PunctuationKind},

    /* Operators e. g. '*', '->' */
    Operators(String),

    /* A sequence of characters */
    Identifier(String),

    /* A single character 'a' => unicode codepoint (32bit). */
    Char(char),

    /* algebraic data type */
    Numeric{raw: String, base: NumericBaseKind, postfix: NumberPostfixKind, form: NumericForm},
}
