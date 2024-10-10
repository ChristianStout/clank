#[allow(dead_code)]
#[derive(Debug)]
pub enum Type {
    I32,
    U8,
    String,
    Char,
    Bool,
    Array(Box<Type>),
    Custom(String),
    Fn(Vec<Type>, Box<Type>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
    If(
        Box<Expr>, // Boolean
        Vec<Stmt>, // Block
    ),
    Else(Vec<Box<Stmt>>), // I'm tempted to remove this 🤔
    For(
        Box<Expr>, // Var
        Box<Expr>, // Iterable
        Vec<Stmt>, // Block
    ),
    While(
        Box<Expr>, //Expr
        Vec<Stmt>, // Block
    ),
    Let(
        String, // Id
        Option<Type>,
        Option<Expr>,
    ),
    When(
        Box<Expr>, // Boolean expression with var
        Vec<Stmt>, // Block
    ),
    // Match(Box<Expr>, Vec<MatchItem>)
    Return(Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    PlusEqual(Box<Expr>, Box<Expr>),
    MinusEqual(Box<Expr>, Box<Expr>),
    DivEqual(Box<Expr>, Box<Expr>),
    MulEqual(Box<Expr>, Box<Expr>),
    ModEqual(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    GtEqual(Box<Expr>, Box<Expr>),
    LtEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    UnaryPos(Box<Expr>),
    UnaryNeg(Box<Expr>),
    Not(Box<Expr>),
    True,
    False,
    Id(String),
    Num(i32),
    Str(String),
    Chr(char),
    RefAccess(Box<Expr>, Box<Expr>),
    NamespaceAccess(Box<Expr>, Box<Expr>),
    FnCall(Box<Expr>, Vec<Expr>),
    Cast(Box<Expr>, Box<Type>),
    ParenExpr(Box<Expr>),
    Lambda(Vec<(String, Option<Type>)>, Vec<Stmt>),
    Range(Box<Expr>, Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TopLevel {
    Fn(
        String,              // Id
        Vec<(String, Type)>, // Params
        Box<Type>,           // Type
        Vec<Stmt>,           // Stmt(s)
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
    Const(
        String,
        Box<Type>,
        Box<Expr>,
    ),
}
