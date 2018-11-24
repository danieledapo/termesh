use crate::Vector3;

#[derive(Debug, PartialEq, Clone)]
pub struct Module<'input> {
    pub input: &'input str,
    pub statements: Vec<Statement<'input>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement<'input> {
    pub line: &'input str,
    pub expr: Expr<'input>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'input> {
    Vertex(Identifier<'input>, Vector3),
    Line(Identifier<'input>, Identifier<'input>),
    Triangle(Identifier<'input>, Identifier<'input>, Identifier<'input>),
}

pub type Identifier<'input> = &'input str;

impl<'input> Module<'input> {
    pub fn vertices(&self) -> impl Iterator<Item = &Vector3> {
        self.statements.iter().flat_map(|s| {
            if let Expr::Vertex(_, v) = &s.expr {
                Some(v)
            } else {
                None
            }
        })
    }

    pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut Vector3> + Captures<'input> {
        self.statements.iter_mut().flat_map(|e| {
            if let Expr::Vertex(_, v) = &mut e.expr {
                Some(v)
            } else {
                None
            }
        })
    }
}

// Trait to inject a lifetime in an `impl Trait`. `impl Trait`s capture only the
// lifetimes that are actually used in the bounds, but sometimes that's not
// enough and we'd like to include other lifetimes too. See
// https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999
pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}
