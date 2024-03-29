use std::str::FromStr;

use crate::Vector3;

use crate::dsl::ast;
use crate::dsl::ast::{Expr, Module, Statement};

pub type Result<'input, T> = std::result::Result<T, ParseError<'input>>;

pub type ParseError<'input> = ast::Error<'input, ParseErrorKind<'input>>;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseErrorKind<'input> {
    Unexpected(&'input str, &'input str),
    UnexpectedEol(&'input str),
    BadNumber(&'input str),
    BadIdentifier(&'input str),
}

pub fn parse_module(input: &str) -> Result<Module> {
    let stmts = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            LineParser {
                line_no: i,
                raw_line: l,
                line: l.split_whitespace(),
            }
            .parse()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Module {
        input,
        statements: stmts,
    })
}

#[derive(Debug)]
struct LineParser<'input, I> {
    raw_line: &'input str,
    line_no: usize,
    line: I,
}

impl<'input, I> LineParser<'input, I>
where
    I: Iterator<Item = &'input str>,
{
    fn next(&mut self, section: &'input str) -> Result<'input, &'input str> {
        match self.line.by_ref().next() {
            None => self.error(ParseErrorKind::UnexpectedEol(section)),
            Some(s) => Ok(s),
        }
    }

    fn parse(&mut self) -> Option<Result<'input, Statement<'input>>> {
        let section = "vertex | line | triangle";

        let ty = self.next(section).ok()?;

        // comments should probably added to the ast
        if ty.starts_with('#') {
            return None;
        }

        let res = match ty {
            "vertex" => self.parse_vertex(),
            "line" => self.parse_line(),
            "triangle" => self.parse_triangle(),
            stmt_start => self.unexpected(stmt_start, section),
        };

        match res {
            Ok(expr) => {
                if let Some(s) = self.line.by_ref().next() {
                    Some(self.unexpected(s, "<eol>"))
                } else {
                    Some(Ok(Statement {
                        expr,
                        line: self.raw_line,
                        line_no: self.line_no,
                    }))
                }
            }
            Err(err) => Some(Err(err)),
        }
    }

    fn parse_vertex(&mut self) -> Result<'input, Expr<'input>> {
        let id = self.parse_id()?;

        self.eat("=")?;

        let x = self.parse_f32()?;
        let y = self.parse_f32()?;
        let z = self.parse_f32()?;

        Ok(Expr::Vertex(id, Vector3::new(x, y, z)))
    }

    fn parse_line(&mut self) -> Result<'input, Expr<'input>> {
        let v0 = self.parse_id()?;
        let v1 = self.parse_id()?;

        Ok(Expr::Line(v0, v1))
    }

    fn parse_triangle(&mut self) -> Result<'input, Expr<'input>> {
        let v0 = self.parse_id()?;
        let v1 = self.parse_id()?;
        let v2 = self.parse_id()?;

        Ok(Expr::Triangle(v0, v1, v2))
    }

    fn parse_id(&mut self) -> Result<'input, &'input str> {
        let id = self.next("identifier")?;

        let mut id_chars = id.chars();

        let valid = id_chars
            .next()
            .map(|start| start.is_alphabetic() && id_chars.all(|c| c.is_alphanumeric()))
            .unwrap_or(false);

        if valid {
            Ok(id)
        } else {
            self.error(ParseErrorKind::BadIdentifier(id))
        }
    }

    fn parse_f32(&mut self) -> Result<'input, f32> {
        let num = self.next("number")?;
        f32::from_str(num).or_else(|_| self.error(ParseErrorKind::BadNumber(num)))
    }

    fn eat(&mut self, what: &'static str) -> Result<'input, ()> {
        let p = self.next(what)?;

        if p != what {
            self.unexpected(p, what)
        } else {
            Ok(())
        }
    }

    fn unexpected<T>(&self, got: &'input str, expected: &'static str) -> Result<'input, T> {
        self.error(ParseErrorKind::Unexpected(got, expected))
    }

    fn error<T>(&self, kind: ParseErrorKind<'input>) -> Result<'input, T> {
        Err(ParseError {
            line: self.raw_line,
            line_no: self.line_no,
            kind,
        })
    }
}

impl<'input> std::fmt::Display for ParseErrorKind<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnexpectedEol(expected) => {
                write!(f, "expected {}, found end of line", expected)
            }
            ParseErrorKind::Unexpected(got, expected) => {
                write!(f, "expected {}, found `{}`", expected, got)
            }
            ParseErrorKind::BadIdentifier(got) => write!(f, "`{}` is not a valid identifier", got),
            ParseErrorKind::BadNumber(got) => write!(f, "`{}` is not a valid number", got),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseErrorKind::*;
    use super::*;

    use crate::dsl::ast::Expr::Vertex;

    #[test]
    fn test_vertex_numbers() {
        assert_eq!(
            parse_module("vertex v1 = 1 -1.42 0.5E-12"),
            Ok(Module {
                input: "vertex v1 = 1 -1.42 0.5E-12",
                statements: vec![Statement {
                    line: "vertex v1 = 1 -1.42 0.5E-12",
                    line_no: 0,
                    expr: Vertex("v1", Vector3::new(1.0, -1.42, 0.5E-12))
                }]
            })
        );
    }

    #[test]
    fn test_unexpected_eol() {
        assert_eq!(
            parse_module("vertex"),
            Err(ast::Error {
                line_no: 0,
                line: "vertex",
                kind: UnexpectedEol("identifier"),
            })
        );

        assert_eq!(
            parse_module("vertex v1 ="),
            Err(ast::Error {
                line_no: 0,
                line: "vertex v1 =",
                kind: UnexpectedEol("number"),
            })
        );

        assert_eq!(
            parse_module("line v1"),
            Err(ast::Error {
                line_no: 0,
                line: "line v1",
                kind: UnexpectedEol("identifier"),
            })
        );
    }

    #[test]
    fn test_unexpected() {
        assert_eq!(
            parse_module("vertex v1 42"),
            Err(ast::Error {
                line_no: 0,
                line: "vertex v1 42",
                kind: Unexpected("42", "="),
            })
        );

        assert_eq!(
            parse_module("line v1 v2 v3"),
            Err(ast::Error {
                line_no: 0,
                line: "line v1 v2 v3",
                kind: Unexpected("v3", "<eol>"),
            })
        );
    }

    #[test]
    fn test_bad_number() {
        assert_eq!(
            parse_module("vertex v = 42a 0 0"),
            Err(ast::Error {
                line_no: 0,
                line: "vertex v = 42a 0 0",
                kind: BadNumber("42a"),
            })
        );

        assert_eq!(
            parse_module("vertex v = 0.98a 6 0"),
            Err(ast::Error {
                line_no: 0,
                line: "vertex v = 0.98a 6 0",
                kind: BadNumber("0.98a"),
            })
        );
    }

    #[test]
    fn test_bad_identifier() {
        assert_eq!(
            parse_module("vertex 42s"),
            Err(ast::Error {
                line_no: 0,
                line: "vertex 42s",
                kind: BadIdentifier("42s"),
            })
        );

        assert_eq!(
            parse_module("line 42 v1"),
            Err(ast::Error {
                line_no: 0,
                line: "line 42 v1",
                kind: BadIdentifier("42"),
            })
        );

        assert_eq!(
            parse_module("triangle v0 v1 1234"),
            Err(ast::Error {
                line_no: 0,
                line: "triangle v0 v1 1234",
                kind: BadIdentifier("1234"),
            })
        );
    }
}
