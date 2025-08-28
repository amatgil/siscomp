//! Non-copy parser
//! The core of the parser is [IResult], which every parsing-related function
//! returns. It can be propagated up with `?` or handled locally to
//! implement alternatives
//!
//! (Note that copying _does_ occur when an error is reached for prettier reporting)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    FunctionDeclaration {
        ret_type: &'a str,
        name: &'a str,
        args: Vec<(&'a str, &'a str)>,
        body: Vec<Statement<'a>>,
    },
}

fn parse_function<'src>(input: &str) -> u8 {
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
