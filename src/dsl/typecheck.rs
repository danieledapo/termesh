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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dsl::parse_module;

    #[test]
    fn test_type_check() {
        let prog = parse_module(
            r"vertex v1 = 2 3 4
              vertex v2 = 3 3 3
              line v1 v2
              vertex v3 = 0 0 0
              triangle v1 v2 v3",
        )
        .unwrap();

        assert_eq!(type_check(&prog), Ok(()));
    }

    #[test]
    fn test_undeclared_var() {
        let prog = parse_module(
            r"vertex v1 = 2 3 4
              line v1 v2",
        )
        .unwrap();

        assert_eq!(
            type_check(&prog),
            Err(ast::Error {
                line_no: 1,
                line: "              line v1 v2",
                kind: TypeCheckErrorKind::UndeclaredVariable("v2")
            })
        );

        let prog = parse_module(
            r"vertex v1 = 2 3 4
              vertex v2 = 1 1 1
              triangle v0 v1 v2",
        )
        .unwrap();

        assert_eq!(
            type_check(&prog),
            Err(ast::Error {
                line_no: 2,
                line: "              triangle v0 v1 v2",
                kind: TypeCheckErrorKind::UndeclaredVariable("v0")
            })
        );
    }
}
