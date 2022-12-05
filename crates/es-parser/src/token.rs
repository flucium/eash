use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    String(String), //
    Ampersand,      // &
    Dollar,         // $
    Equal,          // =
    Pipe,           // |
    Gt,             // >
    Lt,             // <
    Semicolon,      // ;
}

impl Display for Token {
    fn fmt(&self, tkn: &mut Formatter) -> Result {
        match self {
            Token::String(string) => write!(tkn, "{string}"),
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
