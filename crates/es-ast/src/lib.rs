#[derive(Debug)]
pub enum Statement {
    Loop(Loop),
    If(If),
    Assign(Assign),
    Block(Block),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Loop {
    comparison: Comparison,
    block: Block,
}

impl Loop {
    pub fn new(comparison: Comparison) -> Self {
        Self {
            comparison: comparison,
            block: Block {
                left: None,
                right: None,
            },
        }
    }

    pub fn insert(&mut self, block: Block) {
        self.block = block;
    }

    pub fn comparison(&self) -> &Comparison {
        &self.comparison
    }

    pub fn block(&self) -> &Block {
        &self.block
    }
}

#[derive(Debug)]
pub struct If {
    comparison: Comparison,
    block: Block,
    child: Option<Box<Self>>,
}

impl If {
    pub fn new(comparison: Comparison) -> Self {
        Self {
            comparison: comparison,
            block: Block {
                left: None,
                right: None,
            },
            child: None,
        }
    }

    pub fn insert_block(&mut self, block: Block) {
        self.block = block;
    }

    pub fn insert_child(&mut self, child: If) {
        if let Some(self_child) = self.child.as_mut() {
            self_child.insert_child(child)
        } else {
            self.child = Some(Box::new(child))
        }
    }

    pub fn comparison(&self) -> &Comparison {
        &self.comparison
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn child(&self) -> Option<&If> {
        match &self.child {
            Some(child) => Some(&child),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Assign {
    identify: Expression,
    expr: Expression,
}

impl Assign {
    pub fn new(identify: Expression, expr: Expression) -> Self {
        Self {
            identify: identify,
            expr: expr,
        }
    }

    pub fn identify(&self) -> &Expression {
        &self.identify
    }

    pub fn expr(&self) -> &Expression {
        &self.expr
    }
}

#[derive(Debug)]
pub struct Block {
    left: Option<Box<Statement>>,
    right: Option<Box<Statement>>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, statement: Statement) {
        self.left = Some(Box::new(statement));
    }

    pub fn insert_right(&mut self, statement: Statement) {
        self.right = Some(Box::new(statement));
    }

    pub fn left(&self) -> Option<&Statement> {
        match &self.left {
            Some(left) => Some(&left),
            None => None,
        }
    }

    pub fn right(&self) -> Option<&Statement> {
        match &self.right {
            Some(right) => Some(&right),
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    String(String),
    Variable(String),
    Number(isize),
    Background(bool),
    Boolean(bool),
    FD(u32),
    Command(Command),
    Redirect(Redirect),
    Pipe(Pipe),
    Comparison(Comparison),
}

#[derive(Debug)]
pub struct Command {
    prefix: Box<Expression>,
    suffix: Option<CommandSuffix>,
}

impl Command {
    pub fn new(prefix: Expression) -> Self {
        Self {
            prefix: Box::new(prefix),
            suffix: None,
        }
    }

    // pub fn insert_suffix(&mut self, expr: Expression) {
    //     if let Some(suffix) = self.suffix.as_mut() {
    //         suffix.insert(expr);
    //     } else {
    //         let mut suffix = CommandSuffix::new();
    //         suffix.insert(expr);
    //         self.suffix = Some(suffix);
    //     }
    // }

    pub fn insert_suffix(&mut self,suffix:CommandSuffix){
        self.suffix = Some(suffix);
        
    }

    pub fn prefix(&self) -> &Expression {
        &self.prefix
    }

    pub fn suffix(&mut self) -> Option<&CommandSuffix> {
        match &self.suffix {
            Some(suffix) => Some(suffix),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct CommandSuffix {
    expr: Option<Box<Expression>>,
    suffix: Option<Box<Self>>,
}

impl CommandSuffix {
    pub fn new() -> Self {
        Self {
            expr: None,
            suffix: None,
        }
    }

    pub fn insert(&mut self, expr: Expression) {
        if self.expr.is_none() {
            self.expr = Some(Box::new(expr));
        } else if self.suffix.is_none() {
            self.suffix = Some(Box::new(CommandSuffix {
                expr: Some(Box::new(expr)),
                suffix: None,
            }));
        } else {
            if let Some(suffix) = self.suffix.as_mut() {
                suffix.insert(expr);
            }
        }
    }

    pub fn expr(&self) -> Option<&Expression> {
        match &self.expr {
            Some(expr) => Some(&expr),
            None => None,
        }
    }

    pub fn suffix(&self) -> Option<&CommandSuffix> {
        match &self.suffix {
            Some(suffix) => Some(&suffix),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Redirect {
    kind: RedirectKind,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Redirect {
    pub fn new(kind: RedirectKind, left: Expression, right: Expression) -> Self {
        Self {
            kind: kind,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn kind(&self) -> &RedirectKind {
        &self.kind
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}

#[derive(Debug)]
pub enum RedirectKind {
    Write,
    Read,
}

#[derive(Debug)]
pub struct Pipe {
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, expr: Expression) {
        self.left = Some(Box::new(expr));
    }

    pub fn insert_right(&mut self, expr: Expression) {
        self.right = Some(Box::new(expr));
    }

    pub fn left(&self) -> Option<&Expression> {
        match &self.left {
            Some(left) => Some(&left),
            None => None,
        }
    }

    pub fn right(&self) -> Option<&Expression> {
        match &self.right {
            Some(right) => Some(&right),
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum Comparison {
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Gt {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Lt {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
