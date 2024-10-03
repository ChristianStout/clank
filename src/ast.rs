pub enum Type {
    I32,
    U8,
    Char,
    String,
    Bool,
    Custom(String),
    Fn(Vec<Type>, Type),
    // Generic(String, Vec<Type>),
}

pub enum Stmt {
    Let(
        String, // Id
        Optional<Type>,
        Optional<Expr>,
    ),
    For(
        Expr,      // Var
        Expr,      // Iterable
        Vec<Stmt>, // Block
    ),
    While(
        Expr,      // Boolean
        Vec<Stmt>, // Block
    ),
    Expr(Expr),
    If(
        Expr,      // Boolean
        Vec<Stmt>, // Block
    ),
    Else(Vec<Stmt>), // I'm tempted to remove this 🤔
    Return(Expr),
}

pub enum Expr {
    Id(String),
    Num(i32),
    Str(String),
    Chr(char),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    Mul(Expr, Expr),
    Div(Expr, Expr),
    Mod(Expr, Expr),
    UnaryPos(Expr),
    UnaryNeg(Expr),
    True,
    False,
}

pub enum TopLevel {
    Fn(
        String,              // Id
        Vec<(String, Type)>, // Params
        Type,                // Type
        Vec<Stmt>,           // Stmt(s)
    ),
    Struct(
        String,            // Id
        Vec<String, Type>, // StructItem
    ),
    StructItem(
        String, // Id
        Type,   // Type
    ),
    Import(String),
    Const(String, Type, Expr),
}
