pub mod ast;
pub mod parser;
pub mod typecheck;

pub use self::parser::parse_module;
pub use self::typecheck::type_check;

#[cfg(test)]
mod tests {
    use crate::Vector3;

    use super::ast::*;
    use super::*;

    #[test]
    fn test_parse_simple() {
        let module = r"
        vertex v1 = 3 2 1
        vertex v2 = 1 2 3
        vertex v3 = 0 0 0
        vertex v4 = 9 9 9

        line v1 v2
        triangle v2 v3 v4
        ";

        let tree = parse_module(module).unwrap();
        assert_eq!(
            tree,
            Module {
                input: module,
                statements: vec![
                    Statement {
                        line: "        vertex v1 = 3 2 1",
                        line_no: 1,
                        expr: Expr::Vertex("v1", Vector3::new(3.0, 2.0, 1.0))
                    },
                    Statement {
                        line: "        vertex v2 = 1 2 3",
                        line_no: 2,
                        expr: Expr::Vertex("v2", Vector3::new(1.0, 2.0, 3.0))
                    },
                    Statement {
                        line: "        vertex v3 = 0 0 0",
                        line_no: 3,
                        expr: Expr::Vertex("v3", Vector3::new(0.0, 0.0, 0.0))
                    },
                    Statement {
                        line: "        vertex v4 = 9 9 9",
                        line_no: 4,
                        expr: Expr::Vertex("v4", Vector3::new(9.0, 9.0, 9.0))
                    },
                    Statement {
                        line: "        line v1 v2",
                        line_no: 6,
                        expr: Expr::Line("v1", "v2")
                    },
                    Statement {
                        line: "        triangle v2 v3 v4",
                        line_no: 7,
                        expr: Expr::Triangle("v2", "v3", "v4")
                    },
                ]
            }
        );

        assert!(type_check(&tree).is_ok());
    }
}
