use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Token {
    Pipe, // |

    Assign, // =

    Equal,    // ==
    NotEqual, // !=

    Gt, // >
    Lt, // <

    OR,  // ||
    AND, // &&

    Semicolon, // ;
    Comma,     // ,
    Bang,      // !
    Dollar,    // $
    Ampersand, // &

    LParen, // (
    RParen, // )

    LBrace, // {
    RBrace, // }

    Def, // def

    If,   // if
    Elif, // elif
    Else, // else

    Loop, // loop

    Return, // return

    True,  // true
    False, // false

    String(String), // hello
    Ident(String),  // $a , &b
    Number(isize),  // 0 ~ 9
    FD(u32),        // 0 ~ 9
}

impl Display for Token {
    fn fmt(&self, tkn: &mut Formatter) -> Result {
        match self {
            Token::Pipe => write!(tkn, "|"),
            Token::Assign => write!(tkn, "="),
            Token::Equal => write!(tkn, "=="),
            Token::NotEqual => write!(tkn, "!="),
            Token::Gt => write!(tkn, ">"),
            Token::Lt => write!(tkn, "<"),
            Token::OR => write!(tkn, "||"),
            Token::AND => write!(tkn, "&&"),
            Token::Semicolon => write!(tkn, ";"),
            Token::Comma => write!(tkn, ","),
            Token::Bang => write!(tkn, "!"),
            Token::Dollar => write!(tkn, "$"),
            Token::Ampersand => write!(tkn, "&"),
            Token::LParen => write!(tkn, "("),
            Token::RParen => write!(tkn, ")"),
            Token::LBrace => write!(tkn, "{{"),
            Token::RBrace => write!(tkn, "}}"),
            Token::Def => write!(tkn, "def"),
            Token::If => write!(tkn, "if"),
            Token::Elif => write!(tkn, "elif"),
            Token::Else => write!(tkn, "else"),
            Token::Loop => write!(tkn, "loop"),
            Token::Return => write!(tkn, "return"),
            Token::True => write!(tkn, "true"),
            Token::False => write!(tkn, "false"),
            Token::String(v) => write!(tkn, "{v}"),
            Token::Ident(v) => write!(tkn, "{v}"),
            Token::Number(v) => write!(tkn, "{v}"),
            Token::FD(v) => write!(tkn, "{v}"),
        }
    }
}
