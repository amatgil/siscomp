use std::str::FromStr;

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
    TildeEqual,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Bang,
    Equal,
    Ampersand,
    DoubleAmpersand,
    Pipe,
    DoublePipe,
    String(&'src str),
    Keyword(Keyword), // I don't know if they should just be a straight ident...
    Ident(&'src str),
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
        use Token as T;
        fn ret_and_adv<'src>(s: &mut Lexer<'src>, left: char, t: Token<'src>) -> Token<'src> {
            s.byte_pos += left.len_utf8();
            t
        }

        /// Is 'left' followed by 'right'?
        fn followed_by<'src>(
            s: &mut Lexer<'src>,
            next: Option<char>,
            left: char,
            right: char,
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
        let f = |s, next, left, right, yes, no| Some(followed_by(s, next, left, right, yes, no));

        let mut cs = self.rest().chars();
        loop {
            match cs.next()? {
                ws if ws.is_whitespace() => {
                    self.byte_pos += ws.len_utf8();
                    continue;
                }
                '(' => break Some(ret_and_adv(self, '(', T::LeftParen)),
                ')' => break Some(ret_and_adv(self, ')', T::RightParen)),
                '{' => break Some(ret_and_adv(self, '{', T::LeftBrace)),
                '}' => break Some(ret_and_adv(self, '}', T::RightBrace)),
                '[' => break Some(ret_and_adv(self, '[', T::LeftBracket)),
                ']' => break Some(ret_and_adv(self, ']', T::RightBracket)),
                ',' => break Some(ret_and_adv(self, ',', T::Comma)),
                ':' => break Some(ret_and_adv(self, ':', T::Colon)),
                ';' => break Some(ret_and_adv(self, ';', T::Semicolon)),
                '.' => break Some(ret_and_adv(self, '.', T::Dot)),
                '+' => break f(self, cs.next(), '+', '=', T::PlusEqual, T::Plus),
                '-' => break f(self, cs.next(), '-', '=', T::MinusEqual, T::Minus),
                '*' => break f(self, cs.next(), '*', '=', T::StarEqual, T::Star),
                '/' => break f(self, cs.next(), '/', '=', T::StarEqual, T::Star),
                '%' => break f(self, cs.next(), '%', '=', T::PercentEqual, T::Percent),
                '~' => break f(self, cs.next(), '~', '=', T::TildeEqual, T::Tilde),
                '!' => break f(self, cs.next(), '!', '=', T::BangEqual, T::Bang),
                '=' => break f(self, cs.next(), '=', '=', T::EqualEqual, T::Equal),
                '<' => break f(self, cs.next(), '<', '=', T::LessEqual, T::Less),
                '>' => break f(self, cs.next(), '>', '=', T::GreaterEqual, T::Greater),
                '&' => break f(self, cs.next(), '&', '&', T::DoubleAmpersand, T::Ampersand),
                '|' => break f(self, cs.next(), '|', '|', T::DoublePipe, T::Pipe),
                c if c == '_' || c.is_alphabetic() => {
                    let start = self.byte_pos;
                    let mut end = start + c.len_utf8();
                    while let Some(next) = cs.next()
                        && (next.is_alphanumeric() || next == '_')
                    {
                        end += next.len_utf8()
                    }
                    self.byte_pos = end;
                    let ident = &self.whole[start..end];
                    break Some(if let Ok(keyword) = Keyword::from_str(ident) {
                        Token::Keyword(keyword)
                    } else {
                        Token::Ident(ident)
                    });
                }
                _ => break None,
            }
        }
    }
}

#[test]
fn basic_lexing() {
    let s = "x && y;";
    let mut l = Lexer::new(s);
    assert_eq!(Token::Ident("x"), l.next().unwrap());
    assert_eq!(Token::DoubleAmpersand, l.next().unwrap());
    assert_eq!(Token::Ident("y"), l.next().unwrap());
    assert_eq!(Token::Semicolon, l.next().unwrap());
}

#[test]
fn multichar_ident() {
    let s = "hola && adeu || (si % no)";
    let mut l = Lexer::new(s);
    assert_eq!(Token::Ident("hola"), l.next().unwrap());
    assert_eq!(Token::DoubleAmpersand, l.next().unwrap());
    assert_eq!(Token::Ident("adeu"), l.next().unwrap());
    assert_eq!(Token::DoublePipe, l.next().unwrap());

    assert_eq!(Token::LeftParen, l.next().unwrap());
    assert_eq!(Token::Ident("si"), l.next().unwrap());
    assert_eq!(Token::Percent, l.next().unwrap());
    assert_eq!(Token::Ident("no"), l.next().unwrap());
    assert_eq!(Token::RightParen, l.next().unwrap());
}

//l @ '+' => Some(followed_by(
//    self,
//    l,
//    '=',
//    cs.next(),
//    Token::PlusEqual,
//    Token::Plus,
//)),
