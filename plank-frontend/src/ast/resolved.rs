pub use plank_syntax::ast::{BinaryOp, FunctionType, Literal, Number, Signedness, Size, UnaryOp};
use plank_syntax::position::{Span, Spanned};


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
pub struct Symbol(u32);

impl Symbol {
    pub fn new(id: u32) -> Symbol {
        Symbol(id)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Spanned<Expr>>, Spanned<BinaryOp>, Box<Spanned<Expr>>),
    Unary(Spanned<UnaryOp>, Box<Spanned<Expr>>),
    Call(Box<Spanned<Expr>>, Vec<Spanned<Expr>>),
    Field(Box<Spanned<Expr>>, Spanned<String>),
    Name(Spanned<Symbol>, Vec<Spanned<Type>>),
    Literal(Literal),
    Error,
}

#[derive(Debug, Clone)]
pub enum Statement {
    If(
        Spanned<Expr>,
        Box<Spanned<Statement>>,
        Option<Box<Spanned<Statement>>>,
    ),
    Loop(Box<Spanned<Statement>>),
    While(Spanned<Expr>, Box<Spanned<Statement>>),
    Break,
    Continue,
    Return(Spanned<Expr>),
    Let(Spanned<Symbol>, Spanned<Type>, Spanned<Expr>),
    Block(Vec<Spanned<Statement>>),
    Expr(Spanned<Expr>),
}

#[derive(Debug, Clone)]
pub enum Type {
    Wildcard,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    Bool,
    Concrete(Spanned<Symbol>, Vec<Spanned<Type>>),
    Pointer(Box<Spanned<Type>>),
    Function(Vec<Spanned<Type>>, Box<Spanned<Type>>),
    Error,
}

#[derive(Debug, Clone)]
pub struct ItemName {
    pub name: Spanned<Symbol>,
    pub type_params: Vec<Spanned<Symbol>>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub fn_type: FunctionType,
    pub complete_span: Span,
    pub name: ItemName,
    pub params: Vec<Var>,
    pub return_type: Spanned<Type>,
    pub body: Option<Spanned<Statement>>,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Spanned<Symbol>,
    pub typ: Spanned<Type>,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: ItemName,
    pub fields: Vec<Var>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
}