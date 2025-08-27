//! Non-copy parser
//! The core of the parser is [IResult], which every parsing-related function
//! returns. It can be propagated up with `?` or handled locally to
//! implement alternatives
//!
//! (Note that copying _does_ occur when an error is reached for prettier reporting)

use std::error;
use std::error::Error;

/// Ok((input, parsed)), Err(ParseError)
///
/// RetT is the return type, 'i is the input lifetime, 'e is the error lifetime
pub type IResult<'i, E, RetT> = Result<(&'i str, RetT), E>;

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
pub enum ParseError {
    #[error("tag not found")]
    TagNotFound(#[from] TagNotFoundError),
    #[error("end of file unexpectedly encountered")]
    Eof,
    #[error("none of the expected options were encountered")]
    NoAltMatch(#[from] NoAltMatchError),
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("unexpected end of file")]
pub struct EofError;

/// Parse a given program (file) and returns a series of Statements
/// to be executed
pub fn parse<'i>(input: &'i str) -> Result<Vec<Statement<'i>>, ParseError> {
    todo!()
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("expected a symbol")]
struct SymbolNotFoundError;

fn parse_symbol<'i>(input: &'i str) -> IResult<'i, SymbolNotFoundError, &'i str> {
    let input = input.trim_start();
    match take_while(input, char::is_alphabetic) {
        Ok(o) => Ok(o),
        Err(()) => Err(SymbolNotFoundError),
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("expected '{0}'")]
pub struct TagNotFoundError(String);

// Not sure about the lifetime of tag
fn parse_tag<'i>(input: &'i str, tag: &'i str) -> IResult<'i, TagNotFoundError, &'i str> {
    let input = input.trim_start();
    if let Some(rest) = input.strip_prefix(tag) {
        Ok((rest, tag))
    } else {
        Err(TagNotFoundError(tag.to_string()))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum FunctionParseError {
    #[error("no return type found")]
    NoRetType(#[from] NoFnRetTypeError),
    #[error("no function name found")]
    NoName(#[from] NoFnNameError),
    #[error("invalid argument list")]
    InvalidArgs(#[from] InvalidArgsError),
    #[error("invalid body list")]
    InvalidBody(#[from] InvalidBlockError),
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("did not find a return type")]
pub struct NoFnRetTypeError(#[from] SymbolNotFoundError);
#[derive(Debug, Clone, thiserror::Error)]
#[error("did not find a function name")]
pub struct NoFnNameError(#[from] SymbolNotFoundError);

fn parse_function<'i>(input: &'i str) -> IResult<'i, FunctionParseError, Statement<'i>> {
    let input = input.trim_start();
    let (input, ret_type) = parse_symbol(input).map_err(NoFnRetTypeError)?;
    let (input, name) = parse_symbol(input).map_err(NoFnNameError)?;
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum InvalidArgsError {
    #[error(transparent)]
    NoOpenParen(#[from] NoOpenParenError),
    #[error(transparent)]
    NoCloseParen(#[from] NoCloseParenError),
    #[error("type could not be read")]
    MalformedType(#[from] MalformedTypeInArgsList),
    #[error("variable identifier could not be read")]
    MalformedIdent(#[from] MalformedIdentInArgsList),
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("open paren was missing")]
pub struct NoOpenParenError(#[from] TagNotFoundError);
#[derive(Debug, Clone, thiserror::Error)]
#[error("closing paren was missing")]
pub struct NoCloseParenError(#[from] TagNotFoundError);

#[derive(Debug, Clone, thiserror::Error)]
#[error("type was malformed")]
pub struct MalformedTypeInArgsList(#[from] SymbolNotFoundError);

#[derive(Debug, Clone, thiserror::Error)]
#[error("ident was malformed")]
pub struct MalformedIdentInArgsList(#[from] SymbolNotFoundError);

// int a, int* b
fn parse_function_arguments<'i>(
    input: &'i str,
) -> IResult<'i, InvalidArgsError, Vec<(&'i str, &'i str)>> {
    let mut output = vec![];
    let input = input.trim_start();

    let (mut input, _) = parse_tag(input, "(").map_err(NoOpenParenError)?;
    let (ty, ident): (&str, &str);
    loop {
        input = input.trim_start();
        if let Ok((i, _)) = parse_tag(input, ")") {
            input = i;
            break;
        }

        input = input.trim_start();

        (input, ty) = parse_symbol(input).map_err(MalformedTypeInArgsList)?;
        (input, ident) = parse_symbol(input).map_err(MalformedIdentInArgsList)?;

        output.push((ty, ident));

        match parse_tag(input, ",") {
            Ok((i, _)) => input = i,
            Err(_) => break,
        }
        break;
    }

    Ok((input, output))
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("open bracket was missing")]
pub struct NoOpenBracketError(#[from] TagNotFoundError);
#[derive(Debug, Clone, thiserror::Error)]
#[error("closing bracket was missing")]
pub struct NoCloseBracketError(#[from] TagNotFoundError);

#[derive(Debug, Clone, thiserror::Error)]
pub enum InvalidBlockError {
    #[error(transparent)]
    NoOpenBracket(#[from] NoOpenBracketError),
    #[error(transparent)]
    NoCloseBracket(#[from] NoCloseBracketError),
}

fn parse_block<'i>(input: &'i str) -> IResult<'i, InvalidBlockError, Vec<Statement<'i>>> {
    let ret = vec![];
    let input = input.trim_start();
    let (input, _) = parse_tag(input, "{").map_err(NoOpenBracketError)?;

    // TODO: Parse statements here thumbs up emoji

    let (input, _) = parse_tag(input, "}").map_err(NoCloseBracketError)?;

    Ok((input, ret))
}

fn take_while<'i>(input: &'i str, predicate: impl Fn(char) -> bool) -> IResult<'i, (), &'i str> {
    let mut ret = None;
    for (i, c) in input.char_indices() {
        if predicate(c) {
            ret = Some(i + c.len_utf8());
        } else {
            break;
        }
    }
    match ret {
        Some(index) => {
            let (ret, input) = input.split_at(index);
            Ok((input, ret))
        }
        None => Err(()),
    }

    //match input.split_(predicate) {
    //    Some((left, input)) => Ok((input, left)),
    //    None => Err(()),
    //}
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("no match found, expected one of: [{}]", (.0).join(", "))]
pub struct NoAltMatchError(Vec<&'static str>);

/// Tries the alternatives in order, returning the first to succeed
fn alt<'i, RetT>(
    input: &'i str,
    alternatives: &[(
        &'static str,
        impl Fn(&'i str) -> IResult<'i, Box<dyn error::Error>, RetT>,
        // TODO: I don't like the dyn ^here :(
    )],
) -> IResult<'i, NoAltMatchError, RetT> {
    let input = input.trim_start();

    for (_name, alt) in alternatives {
        if let Ok(r) = alt(input) {
            return Ok(r);
        }
    }
    Err(NoAltMatchError(
        alternatives.iter().map(|(name, _)| *name).collect(),
    ))?
}

fn print_error_chain(mut e: &dyn Error) -> String {
    let mut out = String::new();

    out.push_str(&e.to_string());
    out.push_str(": ");

    while let Some(s) = e.source() {
        e = s;
        out.push_str(&e.to_string());
        out.push_str(": ");
    }

    out
}

#[test]
fn empty_function() {
    let source = "void main() {}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(
        program,
        Statement::FunctionDeclaration {
            ret_type: "void",
            name: "main",
            args: vec![],
            body: vec![]
        }
    )
}

#[test]
fn single_declaration() {
    let source = "
void main() {
  int x = 1;
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn double_declaration() {
    let source = "
void main() {
  int x = 1;
  int y = 2;
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn pointer_declaration_left() {
    let source = "
void main() {
  int x = 1;
  int* p = &x;
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn pointer_declaration_right() {
    let source = "
void main() {
  int x = 1;
  int *p = &x;
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn char_pointer_basic() {
    let source = "
void main() {
  char* s = \"Woa a string\";
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn basic_multiple_fn_declaration() {
    let source = "
void do_nothing() {
}

void main() {
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}

#[test]
fn basic_fn_call() {
    let source = "
void do_nothing() {
}

void main() {
   void nothing = do_nothing();
}";
    let (_, program) = match parse_function(source) {
        Ok(o) => o,
        Err(e) => {
            println!("ERROR: {}", print_error_chain(&e));
            panic!("did not parse correctly")
        }
    };
    assert_eq!(program, todo!())
}
