#[allow(dead_code)]
pub enum Type {
    I32,
    U8,
    Char,
    String,
    Bool,
    Custom(String),
    Fn(Vec<Type>, Box<Type>),
}

#[allow(dead_code)]
pub enum Stmt {
    Let(
        String, // Id
        Option<Type>,
        Option<Expr>,
    ),
    For(
        Box<Expr>, // Var
        Box<Expr>, // Iterable
        Vec<Stmt>, // Block
    ),
    While(
        Box<Expr>, // Boolean
        Vec<Stmt>, // Block
    ),
    Expr(Box<Expr>),
    If(
        Box<Expr>, // Boolean
        Vec<Stmt>, // Block
    ),
    Else(Vec<Box<Stmt>>), // I'm tempted to remove this 🤔
    Return(Box<Expr>),
}

#[allow(dead_code)]
pub enum Expr {
    Id(String),
    Num(i32),
    Str(String),
    Chr(char),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    UnaryPos(Box<Expr>),
    UnaryNeg(Box<Expr>),
    True,
    False,
}

#[allow(dead_code)]
pub enum TopLevel {
    Fn(
        String,              // Id
        Vec<(String, Type)>, // Params
        Box<Type>,           // Type
        Vec<Box<Stmt>>,      // Stmt(s)
    ),
    Struct(
        String,              // Id
        Vec<(String, Type)>, // StructItem
    ),
    StructItem(
        String,    // Id
        Box<Type>, // Type
    ),
    Import(String),
    Const(String, Box<Type>, Box<Expr>),
}
