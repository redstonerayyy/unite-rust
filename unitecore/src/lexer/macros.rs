#[macro_export]
macro_rules! create_punctuation_kind {
    (Separator) => {
        $crate::lexer::PunctuationKind::Separator
    };
    (Open $depth:expr) => {
        $crate::lexer::PunctuationKind::Open($depth)
    };
    (Close $depth:expr) => {
        $crate::lexer::PunctuationKind::Close($depth)
    };
}

#[macro_export]
macro_rules! tok {
    (EOF) => {
        $crate::lexer::TokenType::EOF
    };
    (Char $raw:tt) => {
        $crate::lexer::TokenType::Char($raw)
    };
    (Punct $raw:tt ($($inner:tt)+)) => {
        $crate::lexer::TokenType::Punctuation{ raw: $raw, kind: create_punctuation_kind!($(inner) +) }
    }
}