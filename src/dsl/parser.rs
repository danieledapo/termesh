use std::str::FromStr;

use crate::Vector3;

use crate::dsl::ast::{Expr, Module, Statement};

pub type Result<'input, T> = std::result::Result<T, ParseError<'input>>;

#[derive(Debug, PartialEq)]
pub struct ParseError<'input> {
    pub line: &'input str,
    pub kind: ParseErrorKind<'input>,
}

#[derive(Debug, PartialEq)]
pub enum ParseErrorKind<'input> {
    Unexpected(&'input str, &'input str),
    UnexpectedEol(&'input str),
    BadNumber(&'input str),
    BadIdentifier(&'input str),
}

pub fn parse_module(input: &str) -> Result<Module> {
    let stmts = input
        .lines()
        .flat_map(|l| {
            LineParser {
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

        let res = match ty {
            "vertex" => self.parse_vertex(),
            "line" => self.parse_line(),
            "triangle" => self.parse_triangle(),
            stmt_start => self.unexpected(stmt_start, section),
        };

        if let Some(s) = self.line.by_ref().next() {
            Some(self.unexpected(s, "<eol>"))
        } else {
            Some(res.map(|expr| Statement {
                expr,
                line: self.raw_line,
            }))
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
            kind,
        })
    }
}
