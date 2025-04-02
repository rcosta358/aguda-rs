#[derive(Debug)]
pub enum Declaration {
    LetVar(String, Type, Expression),
    LetFun(String, Vec<(String, Type)>, Type, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Num(i64),
    Str(String),
    Bool(bool),
    Null,
    Unit,
    Var(String), // id
    Chain(Box<Expression>, Box<Expression>), // expr; expr
    Let(String, Type, Box<Expression>), // let id : type = expr
    Set(Lhs, Box<Expression>), // set lhs = expr
    BinOp(Box<Expression>, Op, Box<Expression>), // x op y
    While(Box<Expression>, Box<Expression>), // while expr do expr
    IfElse(Box<Expression>, Vec<Expression>, Vec<Expression>), // if expr then expr else expr
    FunCall(String, Box<Expression>), // func(expr)
    ArrayNew(Type, Box<Expression>, Box<Expression>), // new type [size | init]
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
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    String,
    Unit,
    Array(Box<Type>),
    Fun(Vec<Type>, Box<Type>),
}