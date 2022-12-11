use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    String(String), //
    Ident(String),  //
    // Number(isize),  // 0 ~ 9
    FD(i32),
    Ampersand, // &
    Dollar,    // $
    Equal,     // =
    Pipe,      // |
    Gt,        // >
    Lt,        // <
    Semicolon, // ;
}

impl Display for Token {
    fn fmt(&self, tkn: &mut Formatter) -> Result {
        match self {
            Token::String(string) => write!(tkn, "{string}"),
            Token::Ident(string) => write!(tkn, "{string}"),
            // Token::Number(n) => write!(tkn, "{n}"),
            Token::FD(n) => write!(tkn, "{n}"),
            Token::Equal => write!(tkn, "="),
            Token::Ampersand => write!(tkn, "&"),
            Token::Dollar => write!(tkn, "$"),
            Token::Gt => write!(tkn, ">"),
            Token::Lt => write!(tkn, "<"),
            Token::Pipe => write!(tkn, "|"),
            Token::Semicolon => write!(tkn, ";"),
        }
    }
}
