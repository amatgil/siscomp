#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    FunctionDeclaration {
        ret_type: &'a str,
        name: &'a str,
        args: Vec<(&'a str, &'a str)>,
        body: Vec<Statement<'a>>,
    },
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {}

/// Parse a given program (file) and returns a series of Statements
/// to be executed
pub fn parse<'a>(input: &'a str) -> Result<Vec<Statement<'a>>, ParseError> {
    todo!()
}

#[test]
fn empty() {
    let source = "void main() {}";
    let program = parse(source).unwrap();
    assert_eq!(
        program,
        vec![Statement::FunctionDeclaration {
            ret_type: "void",
            name: "main",
            args: vec![],
            body: vec![]
        }]
    )
}
