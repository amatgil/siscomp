//! Non-copy parser
//! The core of the parser is [IResult], which every parsing-related function
//! returns. It can be propagated up with `?` or handled locally to
//! implement alternatives

/// Ok((input, parsed)), Err(ParseError)
///
/// RetT is the return type, 'i is the input lifetime, 'e is the error lifetime
pub type IResult<'i, 'e, RetT> = Result<(&'i str, RetT), ParseError<'e>>;

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
pub enum ParseError<'a> {
    #[error("tag not found: {tag}")]
    TagNotFound { tag: &'a str },
    #[error("end of file unexpectedly encountered")]
    Eof,
    #[error("none of the expected options were encountered: {alternative_errors:?}")]
    NoAlternativeMatched {
        alternative_errors: Vec<ParseError<'a>>,
    },
}

/// Parse a given program (file) and returns a series of Statements
/// to be executed
pub fn parse<'i>(input: &'i str) -> Result<Vec<Statement<'i>>, ParseError<'_>> {
    todo!()
}

fn parse_symbol<'i>(input: &'i str) -> IResult<'i, '_, &'i str> {
    take_while(input, char::is_whitespace)
}

// Not sure about the lifetime of tag
fn parse_tag<'i>(input: &'i str, tag: &'i str) -> IResult<'i, 'i, &'i str> {
    let input = input.trim_start();
    if let Some(rest) = input.strip_prefix(tag) {
        Ok((rest, tag))
    } else {
        Err(ParseError::TagNotFound { tag })
    }
}

fn parse_function<'i>(input: &'i str) -> IResult<'i, '_, Statement> {
    let input = input.trim_start();
    let (input, ret_type) = parse_symbol(input)?;
    let (input, name) = parse_symbol(input)?;
    let (input, args) = parse_function_arguments(input)?;
    let (input, body) = parse_block(input)?;
    Ok((
        input,
        Statement::FunctionDeclaration {
            ret_type,
            name,
            args,
            body,
        },
    ))
}

fn parse_function_arguments<'i>(input: &'i str) -> IResult<'i, 'i, Vec<(&'i str, &'i str)>> {
    let input = input.trim_start();
    todo!()
}

fn parse_block<'i>(input: &'i str) -> IResult<'i, 'i, Vec<Statement>> {
    let input = input.trim_start();
    todo!()
}

fn take_while<'i>(input: &'i str, predicate: impl Fn(char) -> bool) -> IResult<'i, '_, &'i str> {
    match input.split_once(predicate) {
        Some((left, input)) => Ok((input, left)),
        None => Err(todo!("not sure what error goes here")),
    }
}

/// Tries the alternatives in order, returning the first to succeed
fn alt<'i, 'e, RetT>(
    input: &'i str,
    alternatives: &[impl Fn(&'i str) -> IResult<'i, 'e, RetT>],
) -> IResult<'i, 'e, RetT> {
    let input = input.trim_start();

    let mut alternative_errors = vec![];
    for alt in alternatives {
        match alt(input) {
            r @ Ok(_) => return r,
            Err(e) => alternative_errors.push(e),
        }
    }
    Err(ParseError::NoAlternativeMatched { alternative_errors })
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
