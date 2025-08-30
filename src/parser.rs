//! Non-copy parser

use std::str::FromStr;

use crate::lex::*;

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    FunctionDeclaration {
        ty: &'a str,
        name: &'a str,
        args: Vec<(&'a str, &'a str)>,
        body: Vec<Statement<'a>>,
    },
    VarDeclaration {
        ty: &'a str,
        name: &'a str,
        rhs: Expression,
    },
}

// TODO: impl this
#[derive(Debug, Clone)]
pub struct Expression;

impl FromStr for Expression {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Pratt parsing goes here")
    }
}

fn parse<'src>(input: &str) -> Vec<Statement> {
    use Token as T;
    use TokenKind as TK;
    let mut tokens = Lexer::new(input).peekable();

    match tokens.peek() {
        Some(T {
            kind: TK::Keyword(kw),
            ..
        }) => {}
        Some(_) => todo!(),
        None => todo!(),
    }

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
