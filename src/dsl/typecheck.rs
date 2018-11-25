use std::collections::HashSet;

use crate::dsl::ast;

pub type Result<'input, T> = std::result::Result<T, TypeCheckError<'input>>;

pub type TypeCheckError<'input> = ast::Error<'input, TypeCheckErrorKind<'input>>;

#[derive(Debug, PartialEq)]
pub enum TypeCheckErrorKind<'input> {
    UndeclaredVariable(&'input str),
}

pub fn type_check<'input>(module: &ast::Module<'input>) -> Result<'input, ()> {
    let mut env = HashSet::new();

    let has_vertex = |env: &HashSet<&str>, v: &'input str, stmt: &ast::Statement<'input>| {
        if !env.contains(v) {
            Err(TypeCheckError {
                line: stmt.line,
                line_no: stmt.line_no,
                kind: TypeCheckErrorKind::UndeclaredVariable(v),
            })
        } else {
            Ok(())
        }
    };

    for stmt in &module.statements {
        match stmt.expr {
            ast::Expr::Vertex(n, _) => {
                env.insert(n);
            }
            ast::Expr::Line(v0, v1) => {
                has_vertex(&env, v0, stmt)?;
                has_vertex(&env, v1, stmt)?;
            }
            ast::Expr::Triangle(v0, v1, v2) => {
                has_vertex(&env, v0, stmt)?;
                has_vertex(&env, v1, stmt)?;
                has_vertex(&env, v2, stmt)?;
            }
        };
    }

    Ok(())
}

impl<'input> std::fmt::Display for TypeCheckErrorKind<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeCheckErrorKind::UndeclaredVariable(var) => {
                write!(f, "cannot find variable `{}` in this scope", var)
            }
        }
    }
}
