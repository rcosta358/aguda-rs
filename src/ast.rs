#[derive(Debug)]
pub enum Declaration {
    LetVar(String, Type, Expression),
    LetFun(String, Vec<String>, Vec<Type>, Type, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Num(i64),
    Str(String),
    Bool(bool),
    Null,
    Unit,
    Id(String),
    Chain(Box<Expression>, Box<Expression>), // expr; expr
    Let(String, Type, Box<Expression>), // let id : type = expr
    Set(Lhs, Box<Expression>), // set lhs = expr
    BinOp(Box<Expression>, Op, Box<Expression>), // expr op expr
    Not(Box<Expression>), // !expr
    While(Box<Expression>, Box<Expression>), // while expr do expr
    IfElse(Box<Expression>, Box<Expression>, Box<Expression>), // if expr then expr else expr
    FunCall(String, Vec<Expression>), // func(expr, ...)
    NewArray(Type, Box<Expression>, Box<Expression>), // new type [size | init]
}

#[derive(Debug)]
pub enum Lhs {
    Var(String), // var = ...
    Index(Box<Lhs>, Box<Expression>), // array[expr] = ...
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Not,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    String,
    Unit,
    Array(Box<Type>), // type[]
    Fun(Vec<Type>, Box<Type>), // (type, ...) -> type
}