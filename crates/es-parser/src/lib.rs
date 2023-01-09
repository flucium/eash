pub mod error;
pub mod lexer;
pub mod token;
use error::*;
use es_ast::*;
use lexer::*;
use std::iter::Peekable;
use std::mem;
use token::*;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }
    
    fn parse_assign(&mut self) -> Result<Statement> {
        let identify = self.parse_variable()?;

        if self.lexer.next() != Some(Token::Assign) {
            Err(Error::new(ErrorKind::Unknown, "".to_owned()))?
        }

        let expr = self
            .parse_variable()
            .or(self.parse_string().or(self.parse_number()))?;

        Ok(Statement::Assign(Assign::new(identify, expr)))
    }

    fn parse_command(&mut self) -> Result<Expression> {
        
        let mut command = Command::new(
            self.parse_string()
                .or(self.parse_number())
                .or(self.parse_variable())?,
        );

        loop {
            if let Ok(expr) = self.parse_string() {
                command.insert_suffix(expr);
                continue;
            }

            if let Ok(expr) = self.parse_number() {
                command.insert_suffix(expr);
                continue;
            }

            if let Ok(expr) = self.parse_variable() {
                command.insert_suffix(expr);
                continue;
            }

            if let Ok(expr) = self.parse_redirect() {
                command.insert_suffix(expr);
                continue;
            }

            break;
        }

        Ok(Expression::Command(command))
    }

    fn parse_redirect(&mut self) -> Result<Expression> {
        let left = self.parse_fd()?;

        let kind = match self.lexer.next() {
            Some(Token::Gt) => RedirectKind::Write,
            Some(Token::Lt) => RedirectKind::Read,
            _ => Err(Error::new(ErrorKind::Unknown, "".to_owned()))?,
        };

        let right = self
            .parse_string()
            .or(self.parse_number())
            .or(self.parse_variable())?;

        Ok(Expression::Redirect(Redirect::new(kind, left, right)))
    }

    fn parse_fd(&mut self) -> Result<Expression> {
        match self.next_token(Token::FD(0)) {
            Some(Token::FD(number)) => Ok(Expression::FD(number)),
            _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
        }
    }

    fn parse_number(&mut self) -> Result<Expression> {
        match self.next_token(Token::Number(0)) {
            Some(Token::Number(number)) => Ok(Expression::Number(number)),
            _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
        }
    }

    fn parse_variable(&mut self) -> Result<Expression> {
        match self.next_token(Token::Ident(String::default())) {
            Some(Token::Ident(string)) => Ok(Expression::Variable(string)),
            _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
        }
    }

    fn parse_string(&mut self) -> Result<Expression> {
        match self.next_token(Token::String(String::default())) {
            Some(Token::String(string)) => Ok(Expression::String(string)),
            _ => Err(Error::new(ErrorKind::Unknown, "".to_owned())),
        }
    }

    fn next_token(&mut self, token: Token) -> Option<Token> {
        self.lexer
            .next_if(|tkn| mem::discriminant(tkn) == mem::discriminant(&token))
    }
}

// #[macro_export]
// macro_rules! parse_assign {
//     ($($x:tt)*) => {};
// }

// #[macro_export]
// macro_rules! parse_command {

//     ($($x:tt)*)=>{{

//         if $($x)*.is_empty(){
//             return
//         }

//         let mut lexer = $crate::lexer::Lexer::new($($x)*);

//         let prefix=match lexer.next(){
//             Some($crate::token::Token::String(string))=>{
//                 es_ast::Expression::String(string.to_owned())
//             }

//             Some($crate::token::Token::Ident(ident))=>{
//                 es_ast::Expression::Variable(ident.to_owned())
//             }

//             Some($crate::token::Token::Number(number))=>{
//                 es_ast::Expression::Number(number.to_owned())
//             }

//             Some($crate::token::Token::EOL)|Some($crate::token::Token::EOF)=>{
//                 return
//             }

//             _=>{
//                 panic!("")
//             }
//         };

//         let mut command = es_ast::Command::new(prefix);

//         while let Some(token) = lexer.next(){
//             match token{
//                 $crate::token::Token::String(string) => {
//                     command.insert_suffix(es_ast::Expression::String(string.to_owned()));
//                 }
//                 $crate::token::Token::Ident(ident) => {
//                     command.insert_suffix(es_ast::Expression::Variable(ident.to_owned()));
//                 }
//                 $crate::token::Token::Number(number) => {
//                     command.insert_suffix(es_ast::Expression::Number(number.to_owned()));
//                 }
//                 $crate::token::Token::Ampersand => {
//                     command.insert_suffix(es_ast::Expression::Background(true));
//                     break;
//                 }

//                 $crate::token::Token::FD(fd)=>{}

//                 $crate::token::Token::Gt=>{}

//                 $crate::token::Token::Lt=>{}

//                 $crate::token::Token::Semicolon | $crate::token::Token::Pipe | $crate::token::Token::EOL | $crate::token::Token::EOF => break,

//                 _=> {
//                     panic!("")
//                 }
//             }
//         }

//         es_ast::Expression::Command(command)
//     }};
// }
