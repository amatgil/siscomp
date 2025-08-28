use std::str::{Chars, FromStr};

struct Lexer<'src> {
    whole: &'src str,
    /// Must ALWAYS exist outside of a utf8 boundary, i.e. between chars
    byte_pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    Semicolon,
    Tilde, // one's complement!
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Bang,
    Equal,
    String(&'src str),
    Keyword(Keyword), // I don't know if they should just be a straight ident...
    Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    KwAuto,
    KwBreak,
    KwCase,
    KwChar,
    KwConst,
    KwContinue,
    KwDefault,
    KwDo,
    KwDouble,
    KwElse,
    KwEnum,
    KwExtern,
    KwFloat,
    KwFor,
    KwGoto,
    KwIf,
    KwInt,
    KwLong,
    KwRegister,
    KwReturn,
    KwShort,
    KwSigned,
    KwSizeof,
    KwStatic,
    KwStruct,
    KwSwitch,
    KwTypedef,
    KwUnion,
    KwUnsigned,
    KwVoid,
    KwVolatile,
    KwWhile,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::KwAuto),
            "break" => Ok(Self::KwBreak),
            "case" => Ok(Self::KwCase),
            "char" => Ok(Self::KwChar),
            "const" => Ok(Self::KwConst),
            "continue" => Ok(Self::KwContinue),
            "default" => Ok(Self::KwDefault),
            "do" => Ok(Self::KwDo),
            "double" => Ok(Self::KwDouble),
            "else" => Ok(Self::KwElse),
            "enum" => Ok(Self::KwEnum),
            "extern" => Ok(Self::KwExtern),
            "float" => Ok(Self::KwFloat),
            "for" => Ok(Self::KwFor),
            "goto" => Ok(Self::KwGoto),
            "if" => Ok(Self::KwIf),
            "int" => Ok(Self::KwInt),
            "long" => Ok(Self::KwLong),
            "register" => Ok(Self::KwRegister),
            "return" => Ok(Self::KwReturn),
            "short" => Ok(Self::KwShort),
            "signed" => Ok(Self::KwSigned),
            "sizeof" => Ok(Self::KwSizeof),
            "static" => Ok(Self::KwStatic),
            "struct" => Ok(Self::KwStruct),
            "switch" => Ok(Self::KwSwitch),
            "typedef" => Ok(Self::KwTypedef),
            "union" => Ok(Self::KwUnion),
            "unsigned" => Ok(Self::KwUnsigned),
            "void" => Ok(Self::KwVoid),
            "volatile" => Ok(Self::KwVolatile),
            "while" => Ok(Self::KwWhile),
            _ => Err(()),
        }
    }
}

impl<'src> Lexer<'src> {
    fn new(input: &'src str) -> Self {
        Self {
            whole: input,
            byte_pos: 0,
        }
    }
    fn rest(&self) -> &'src str {
        &self.whole[self.byte_pos..]
    }

    /// Returns (line, column)
    fn get_line_and_col(&self) -> (u64, u64) {
        todo!()
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        fn ret_and_adv<'src>(s: &mut Lexer<'src>, len: usize, t: Token<'src>) -> Token<'src> {
            s.byte_pos += len;
            t
        }

        /// Is 'left' followed by 'right'?
        fn followed_by<'src>(
            s: &mut Lexer<'src>,
            left: char,
            right: char,
            next: Option<char>,
            yes: Token<'src>,
            no: Token<'src>,
        ) -> Token<'src> {
            match next {
                Some(n) if n == right => {
                    s.byte_pos += left.len_utf8() + right.len_utf8();
                    yes
                }
                _ => {
                    s.byte_pos += left.len_utf8();
                    no
                }
            }
        }

        let mut cs = self.rest().chars();
        match cs.next()? {
            '(' => Some(ret_and_adv(self, 1, Token::LeftParen)),
            ')' => Some(ret_and_adv(self, 1, Token::RightParen)),
            '{' => Some(ret_and_adv(self, 1, Token::LeftBrace)),
            '}' => Some(ret_and_adv(self, 1, Token::RightBrace)),
            '[' => Some(ret_and_adv(self, 1, Token::LeftBracket)),
            ']' => Some(ret_and_adv(self, 1, Token::RightBracket)),
            ',' => Some(ret_and_adv(self, 1, Token::Comma)),
            ':' => Some(ret_and_adv(self, 1, Token::Colon)),
            ';' => Some(ret_and_adv(self, 1, Token::Semicolon)),
            '.' => Some(ret_and_adv(self, 1, Token::Dot)),
            l @ '+' => Some(followed_by(
                self,
                l,
                '=',
                cs.next(),
                Token::PlusEqual,
                Token::Plus,
            )),
            l @ '-' => Some(followed_by(
                self,
                l,
                '=',
                cs.next(),
                Token::MinusEqual,
                Token::Minus,
            )),
            l @ '*' => Some(followed_by(
                self,
                l,
                '=',
                cs.next(),
                Token::StarEqual,
                Token::Star,
            )),
            l @ '/' => Some(followed_by(
                self,
                l,
                '=',
                cs.next(),
                Token::SlashEqual,
                Token::Slash,
            )),
            //l @ '%' => Some(followed_by(
            //    self,
            //    l,
            //    '=',
            //    &mut cs,
            //    Token::PercentEqual,
            //    Token::Percent,
            //)),
            _ => todo!(), //'-' =>
                          //'*' => Some(Token::Star),
                          //'/' => {}
                          //'~' => Some(Token::Tilde),
                          //'!' => {}
                          //'<' => {}
                          //'>' => {}
                          //'=' => {}
                          //'_' => {}
                          //'&' => {}
        }
    }
}

//l @ '+' => Some(followed_by(
//    self,
//    l,
//    '=',
//    cs.next(),
//    Token::PlusEqual,
//    Token::Plus,
//)),
