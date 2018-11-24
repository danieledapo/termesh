use crate::Vector3;

#[derive(Debug, PartialEq)]
pub struct Module<'input> {
    pub input: &'input str,
    pub statements: Vec<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct Statement<'input> {
    pub line: &'input str,
    pub expr: Expr<'input>,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'input> {
    Vertex(Identifier<'input>, Vector3),
    Line(Identifier<'input>, Identifier<'input>),
    Triangle(Identifier<'input>, Identifier<'input>, Identifier<'input>),
}

pub type Identifier<'input> = &'input str;
