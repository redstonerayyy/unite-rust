extern crate unitecore;    

use unitecore::lexer::*;

fn main() {
    let mut lexer = Lexer::new("2.2 2.2e92 .2");

    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => break,
            Ok(tok) => println!("{0:?}", tok),
            Err(err) => println!("{0:?}", err),
        }
    }
}
