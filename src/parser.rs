//! Non-copy parser

use std::{iter, str::FromStr};

use crate::lex::*;

#[derive(Debug, Clone)]
pub enum Ast<'a> {
    Empty,
    FunctionDeclaration {
        ty: &'a str,
        name: &'a str,
        args: Vec<(&'a str, &'a str)>,
        body: Vec<Ast<'a>>,
    },
    VarDeclaration {
        ty: &'a str,
        name: &'a str,
        rhs: Expression,
    },
    Atom(Atom<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom<'a> {
    String(&'a str),
    Integer(u128),
    Float(f64),
    Keyword(Keyword),
    Ident(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomError<'src> {
    NotAnAtom,
    InvalidInteger(&'src str),
    InvalidFloat(&'src str),
}

impl<'a> TryFrom<Token<'a>> for Atom<'a> {
    type Error = AtomError<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::String(s) => Ok(Self::String(s)),
            TokenKind::Integer(s) if s.parse::<u128>().is_ok() => {
                Ok(Self::Integer(s.parse().unwrap()))
            }
            TokenKind::Integer(s) => Err(AtomError::InvalidInteger(s)),

            TokenKind::Float(f) if f.parse::<u128>().is_ok() => Ok(Self::Float(f.parse().unwrap())),
            TokenKind::Keyword(kw) => Ok(Self::Keyword(kw)),
            TokenKind::Ident(i) => Ok(Self::Ident(i)),
            _ => Err(AtomError::NotAnAtom),
        }
    }
}

type L<'src> = iter::Peekable<Lexer<'src>>;

#[derive(Debug, Clone, Copy)]
pub struct ParseError<'src> {
    whole: &'src str,
    byte_pos: usize,
    kind: ParseErrorKind,
}
#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    LexErr(LexerErrorKind),
}

impl<'src> From<LexerError<'src>> for ParseError<'src> {
    fn from(le: LexerError<'src>) -> Self {
        ParseError {
            whole: le.whole,
            byte_pos: le.byte_pos,
            kind: ParseErrorKind::LexErr(le.kind),
        }
    }
}

// TODO: impl this
#[derive(Debug, Clone)]
pub struct Expression;

fn parse<'src>(input: &'src str) -> Result<Ast<'src>, ParseError<'src>> {
    use Token as T;
    use TokenKind as TK;
    let mut tokens = Lexer::new(input).peekable();

    match tokens.peek().copied().transpose()? {
        Some(t) if Atom::try_from(t) != Err(AtomError::NotAnAtom) => {
            Ok(Ast::Atom(Atom::try_from(t).unwrap()))
        }
        Some(T {
            kind: TK::ParenOpen,
            ..
        }) => todo!(),
        Some(_) => todo!(),
        None => return Ok(Ast::Empty),
    }
}

fn parse_expression(lex: &mut L) -> Result<Expression, u8> {
    use TokenKind as TK;
    /// Binding power (left, right)
    /// String binding power on that side means it associates on the
    /// other side
    ///
    /// Returns None if `op` is not an infix operator
    fn prefix_binding_power(op: TokenKind) -> Option<((), u8)> {
        match op {
            // RTL Unary
            TK::PreIncrement
            | TK::PreDecrement
            | TK::Bang
            | TK::Tilde
            | TK::Star
            | TK::Ampersand
            | TK::Keyword(Keyword::KwSizeof) => todo!(), // RTL unary
            _ => None,
        }
    }
    fn infix_binding_power(op: TokenKind) -> Option<(u8, u8)> {
        match op {
            //TK::ParenOpen | TK::ParenClose
            //TK::BracketOpen | TK::BracketClose

            // LTR strongest
            TK::Dot | TK::RightArrow => todo!(), // LTR

            // LTR Strong arith
            TK::Star | TK::Slash | TK::Percent => todo!(),

            // LTR Weak arith
            TK::Plus | TK::Minus => todo!(),

            _ => None,
        }
    }
    fn postfix_binding_power(op: TokenKind) -> Option<(u8, ())> {
        match op {
            TK::PostIncrement | TK::PostDecrement => todo!(),
            _ => None,
        }
    }
    //let lhs = match lex.next() {TK};
    todo!()
}

//#[test]
//fn empty_function() {
//    let source = "void main() {}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(
//        program,
//        Statement::FunctionDeclaration {
//            ret_type: "void",
//            name: "main",
//            args: vec![],
//            body: vec![]
//        }
//    )
//}
//
//#[test]
//fn single_declaration() {
//    let source = "
//void main() {
//  int x = 1;
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn double_declaration() {
//    let source = "
//void main() {
//  int x = 1;
//  int y = 2;
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn pointer_declaration_left() {
//    let source = "
//void main() {
//  int x = 1;
//  int* p = &x;
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn pointer_declaration_right() {
//    let source = "
//void main() {
//  int x = 1;
//  int *p = &x;
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn char_pointer_basic() {
//    let source = "
//void main() {
//  char* s = \"Woa a string\";
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn basic_multiple_fn_declaration() {
//    let source = "
//void do_nothing() {
//}
//
//void main() {
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
//
//#[test]
//fn basic_fn_call() {
//    let source = "
//void do_nothing() {
//}
//
//void main() {
//   void nothing = do_nothing();
//}";
//    let (_, program) = match parse_function(source) {
//        Ok(o) => o,
//        Err(e) => {
//            println!("ERROR: {}", print_error_chain(&e));
//            panic!("did not parse correctly")
//        }
//    };
//    assert_eq!(program, todo!())
//}
