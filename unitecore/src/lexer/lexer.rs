use crate::lexer::*;

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

    fn consume_digit(&mut self, raw: &String, for_radix: u32) -> Result<char, LexerError> {
        match self.chars.next() {
            // maybe improve
            None => {
                println!("3");
                Err(LexerError::NumericLiteralInvalidChar { raw: raw.to_string() })
            },
            Some(c) if !c.is_digit(for_radix) => {
                println!("2");
                Err(LexerError::NumericLiteralInvalidChar { raw: raw.to_string() })
            },
            Some(c) => Ok(c)
        }
    }

    fn parse_number(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut seen_dot = false;
        let mut seen_exp = false;
        let mut num = start.to_string();
        let radix = 10;

        if start == '.' {
            num.push(self.consume_digit(&num, 10)?);
            seen_dot = true;
        }

        loop {
            // types of numbers: 1020, .1203, 1.234, 1e+3 1.23e-3 
            match self.chars.peek() {
                Some(c) if *c == '.' && !seen_dot && !seen_exp => {
                    num.push(*c);
                    self.consume_char();
                    seen_dot = true;  
                },
                Some(c) if (*c == 'e' || *c == 'E') && !seen_exp => {
                    num.push(*c);
                    self.consume_char();
                    seen_exp = true;
                    
                     let exp_radix = 10;

                    match self.chars.peek() {
                        Some(c) if *c == '+' || *c == '-' => {
                            num.push(*c);
                            self.consume_char();
                        }
                        _ => {}
                    }
                    
                    num.push(self.consume_digit(&num, exp_radix)?);
                },
                Some(c) if c.is_digit(radix) => {
                    num.push(*c);
                    self.consume_char();
                    
                },
                Some(c) if c.is_ascii_alphabetic() || c.is_digit(10) => {
                    // is_digit(10) because maybe radix is != 10, so fail on 4 if its binary
                    num.push(*c);
                    println!("1");
                    break Err(LexerError::NumericLiteralInvalidChar { raw: num })
                },
                _ => {
                    // exit
                    break Ok(TokenType::Numeric { raw: num, hint: if seen_dot || seen_exp { NumericHint::FloatingPoint} else { NumericHint::Integer} });
                }
            }
        }


    }

    fn transform_to_type(&mut self,c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Open(self.push_symbol(&c)) }),
            ')' | ']' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?)}),
            '0' ..= '9' | '.'  => self.parse_number(c),
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
