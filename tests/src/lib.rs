// turn off unused import because the imports for macros, which are needed,
// get flagged as unused
#![allow(unused_imports)]

extern crate unitecore;

use unitecore::tok;
use unitecore::lexer::*;
use unitecore::lexer::lexer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macros() {
        assert_eq!(tok!(EOF), TokenType::EOF);
        // not working assert_eq!(tok!(Punct '(' (Open 0)), TokenType::Punctuation{ raw: '(', kind: PunctuationKind::Open(0)});
    }
}
