pub mod lexer;
pub mod token;
use crate::lexer::Lexer;
use crate::token::Token;
use es_error::Error;
use es_error::ErrorKind;
use es_error::Result;
use std::iter::Peekable;

pub type Pipe = Tree;

pub type CommandSuffix = Tree;

pub type Block = Tree;

pub struct Parser {
    lexer: Peekable<Lexer>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn parse_command(&mut self) -> Result<Option<Node>> {
        let prefix = match self.parse_command_prefix()? {
            Some(prefix) => prefix,
            None => return Ok(None),
        };

        let suffix = self.parse_command_suffix()?;

        let mut command = Command::new();
        command.insert_prefix(prefix);
        command.insert_suffix(suffix);

        Ok(Some(Node::Command(command)))
    }

    fn parse_command_suffix(&mut self) -> Result<CommandSuffix> {
        let mut suffix = CommandSuffix::new();

        loop {
            if let Some(peek_token) = self.lexer.peek() {
                if matches!(peek_token, Token::Pipe | Token::Semicolon) {
                    break;
                }
            } else {
                break;
            }

            if let Some(node) = self.parse_variable() {
                suffix.insert(node)
            }

            if let Some(node) = self.parse_string().or_else(|| self.parse_number()) {
                suffix.insert(node)
            }

            if let Some(node) = self.parse_redirect()? {
                suffix.insert(node)
            }

            if let Some(node) = self.parse_background() {
                suffix.insert(node);
                break;
            }
        }

        Ok(suffix)
    }

    fn parse_command_prefix(&mut self) -> Result<Option<Node>> {
        match self
            .parse_variable()
            .or_else(|| self.parse_string().or_else(|| self.parse_number()))
        {
            Some(prefix) => Ok(Some(prefix)),
            None => Ok(None),
        }
    }

    fn parse_background(&mut self) -> Option<Node> {
        match self.lexer.next_if_eq(&Token::Ampersand).is_some() {
            true => Some(Node::Background(true)),
            false => None,
        }
    }

    fn parse_variable(&mut self) -> Option<Node> {
        match self.lexer.next_if(|token| matches!(token, Token::Ident(_))) {
            Some(token) => match token {
                Token::Ident(string) => Some(Node::Variable(string)),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_string(&mut self) -> Option<Node> {
        match self
            .lexer
            .next_if(|token| matches!(token, Token::String(_)))
        {
            Some(token) => match token {
                Token::String(string) => Some(Node::String(string)),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_number(&mut self) -> Option<Node> {
        match self
            .lexer
            .next_if(|token| matches!(token, Token::Number(_)))
        {
            Some(token) => match token {
                Token::Number(n) => Some(Node::Number(n)),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_fd(&mut self) -> Option<Node> {
        match self.lexer.next_if(|token| matches!(token, Token::FD(_))) {
            Some(token) => match token {
                Token::FD(n) => Some(Node::FD(n)),
                _ => None,
            },
            None => None,
        }
    }

    fn parse_redirect(&mut self) -> Result<Option<Node>> {
        let mut kind: Option<RedirectKind> = None;
        let mut left: Option<Node> = None;
        let mut right: Option<Node> = None;

        match self.parse_fd() {
            Some(node) => left = Some(node),
            None => match self.lexer.peek() {
                Some(Token::Gt) => left = Some(Node::FD(1)),
                Some(Token::Lt) => left = Some(Node::FD(0)),
                _ => return Ok(None),
            },
        }

        match self.lexer.next() {
            Some(Token::Gt) => kind = Some(RedirectKind::Write),
            Some(Token::Lt) => kind = Some(RedirectKind::Read),
            _ => Err(Error::new(ErrorKind::IllegalSyntax, "".to_owned()))?,
        }

        match self
            .parse_string()
            .or_else(|| self.parse_number().or_else(|| self.parse_fd()))
        {
            Some(node) => right = Some(node),
            None => Err(Error::new(ErrorKind::IllegalSyntax, "".to_owned()))?,
        }

        let mut redirect = Redirect::new(kind.unwrap());
        redirect.insert_left(left.unwrap());
        redirect.insert_right(right.unwrap());
        Ok(Some(Node::Redirect(redirect)))
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    String(String),
    Variable(String),
    Number(isize),
    FD(u32),
    Assignment(Assignment),
    Redirect(Redirect),
    Command(Command),
    Background(bool),
    Block(Block),
    Pipe(Pipe),
}

#[derive(Debug, Clone)]
pub struct Redirect {
    kind: RedirectKind,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Redirect {
    pub fn new(kind: RedirectKind) -> Self {
        Self {
            kind: kind,
            left: None,
            right: None,
        }
    }

    pub fn kind(&self) -> &RedirectKind {
        &self.kind
    }

    pub fn take_left(&mut self) -> Option<Node> {
        match self.left.take() {
            Some(node) => Some(*node),
            None => None,
        }
    }
    pub fn take_right(&mut self) -> Option<Node> {
        match self.right.take() {
            Some(node) => Some(*node),
            None => None,
        }
    }

    fn insert_right(&mut self, node: Node) {
        self.right = Some(Box::new(node))
    }

    fn insert_left(&mut self, node: Node) {
        self.left = Some(Box::new(node))
    }
}

#[derive(Debug, Clone)]
pub enum RedirectKind {
    Write,
    Read,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    ident: Option<Box<Node>>,
    value: Option<Box<Node>>,
}

impl Assignment {
    pub fn new() -> Self {
        Self {
            ident: None,
            value: None,
        }
    }

    pub fn take_ident(&mut self) -> Option<Node> {
        match self.ident.take() {
            Some(node) => Some(*node),
            None => None,
        }
    }

    pub fn take_value(&mut self) -> Option<Node> {
        match self.value.take() {
            Some(node) => Some(*node),
            None => None,
        }
    }

    fn insert_ident(&mut self, node: Node) {
        self.ident = Some(Box::new(node))
    }

    fn insert_value(&mut self, node: Node) {
        self.value = Some(Box::new(node))
    }
}

// #[derive(Debug, Clone)]
// pub struct Redirect {
//     kind: RedirectKind,
// }

// #[derive(Debug, Clone)]
// pub enum RedirectKind {
//     Read,
//     Write,
//     Append,
// }

#[derive(Debug, Clone)]
pub struct Command {
    prefix: Option<Box<Node>>,
    suffix: Option<Box<CommandSuffix>>,
}

impl Command {
    fn new() -> Self {
        Self {
            prefix: None,
            suffix: None,
        }
    }

    pub fn take_prefix(&mut self) -> Option<Node> {
        match self.prefix.to_owned() {
            Some(node) => Some(*node),
            None => None,
        }
    }

    pub fn take_suffix(&mut self) -> Option<CommandSuffix> {
        match self.suffix.take() {
            Some(suffix) => Some(*suffix),
            None => None,
        }
    }

    fn insert_prefix(&mut self, prefix: Node) {
        self.prefix = Some(Box::new(prefix))
    }

    fn insert_suffix(&mut self, suffix: CommandSuffix) {
        if suffix.node.is_some() {
            self.suffix = Some(Box::new(suffix))
        }
        
    }
}

// used with Root/Tree and Pipe, CommandSuffix.
// use it when creating a structure that does not require a large heap memory like Vector(Vec etc..),
// and where the left is a meaningful node and the right falls unilaterally.
// since it is a FIFO, do not use it for structures that make the stack absolute.
// can't Insert or Get or Remove from any position.
#[derive(Debug, Clone)]
pub struct Tree {
    node: Option<Box<Node>>,
    child: Option<Box<Tree>>,
}

impl Tree {
    fn new() -> Self {
        Self {
            node: None,
            child: None,
        }
    }

    pub fn is_child(&self) -> bool {
        if self.node.is_some() {
            return self.child.is_some();
        }

        if let Some(node) = self.child.as_ref() {
            if node.node.is_some() == false {
                return node.is_child();
            } else {
                return true;
            }
        }

        false
    }

    pub fn take(&mut self) -> Option<Node> {
        if let Some(node) = self.node.take() {
            return Some(*node);
        }

        if let Some(node) = self.child.as_mut() {
            return node.take();
        }

        None
    }

    fn insert(&mut self, node: Node) {
        if self.node.is_none() {
            self.node = Some(Box::new(node))
        } else if self.child.is_none() {
            self.child = Some(Box::new(Tree {
                node: Some(Box::new(node)),
                child: None,
            }))
        } else {
            if let Some(child) = self.child.as_mut() {
                child.insert(node)
            }
        }
    }
}
