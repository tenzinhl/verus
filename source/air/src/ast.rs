use std::rc::Rc;
use std::fmt::Debug;

pub type RawSpan = Rc<dyn std::any::Any>;
#[derive(Clone)]
pub struct Span {
    pub raw_span: RawSpan,
    pub as_string: String,
}
pub type SpanOption = Rc<Option<Span>>;

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("Span").field(&self.as_string).finish()
    }
}

#[derive(Debug)]
pub enum ValidityResult {
    Valid,
    Error(SpanOption),
}

pub type Ident = Rc<String>;

#[derive(Debug)]
pub enum Typ {
    Bool,
    Int,
}

#[derive(Copy, Clone, Debug)]
pub enum Const {
    Bool(bool),
}

#[derive(Copy, Clone, Debug)]
pub enum UnaryOp {
    Not,
}

#[derive(Copy, Clone, Debug)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Copy, Clone, Debug)]
pub enum BinaryOp {
    Implies,
    Eq,
    Le,
    Ge,
    Lt,
    Gt,
    Add,
    Sub,
    Mul,
    EuclideanDiv,
    EuclideanMod,
}

pub type Expr = Rc<ExprX>;
pub type Exprs = Rc<Box<[Expr]>>;
#[derive(Debug)]
pub enum ExprX {
    Const(Const),
    Var(Ident),
    Unary(UnaryOp, Expr),
    Binary(BinaryOp, Expr, Expr),
    Logical(LogicalOp, Exprs),
    LabeledAssertion(SpanOption, Expr),
}

pub type Stmt = Rc<StmtX>;
pub type Stmts = Rc<Box<[Stmt]>>;
#[derive(Debug)]
pub enum StmtX {
    Assume(Expr),
    Assert(SpanOption, Expr),
    Block(Stmts),
}

pub type Declaration = Rc<DeclarationX>;
pub type Declarations = Rc<Box<[Declaration]>>;
#[derive(Debug)]
pub enum DeclarationX {
    Const(Ident, Typ),
    Axiom(Expr),
}

pub type Query = Rc<QueryX>;
#[derive(Debug)]
pub struct QueryX {
    pub local: Declarations, // local declarations
    pub assertion: Stmt,     // checked by SMT with global and local declarations
}

pub type Command = Rc<CommandX>;
pub type Commands = Rc<Box<[Command]>>;
#[derive(Debug)]
pub enum CommandX {
    Push,                // push space for temporary global declarations
    Pop,                 // pop temporary global declarations
    Global(Declaration), // global declarations
    CheckValid(Query),   // SMT check-sat (reporting validity rather than satisfiability)
}
