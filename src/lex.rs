use std::str::FromStr;

pub struct Lexer<'src> {
    whole: &'src str,
    /// Must ALWAYS exist outside of a utf8 boundary, i.e. between chars
    byte_pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub byte_start: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'src> {
    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
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
    DoubleAmpersnd,
    Pipe,
    DoublePipe,
    String(&'src str),
    /// We don't know the precision yet, so we keep the whole source
    Integer(&'src str),
    /// We don't know the precision yet, so we keep the whole source
    Float(&'src str),
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
    pub fn new(input: &'src str) -> Self {
        Self {
            whole: input,
            byte_pos: 0,
        }
    }
    fn rest(&self) -> &'src str {
        &self.whole[self.byte_pos..]
    }

    /// Returns (line, column)
    pub fn get_line_and_col(&self) -> (u64, u64) {
        let mut line = 0;
        let mut col = 0;
        for (i, c) in self.whole.char_indices() {
            col += 1;
            if i >= self.byte_pos {
                break;
            } else if c == '\n' {
                line += 1;
                col = 0;
            }
        }

        (line, col)
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        use TokenKind as TK;
        fn ret_and_adv<'src>(s: &mut Lexer<'src>, left: char, t: TokenKind<'src>) -> Token<'src> {
            let byte_start = s.byte_pos;
            s.byte_pos += left.len_utf8();
            Token {
                kind: t,
                byte_start,
            }
        }

        /// Is 'left' followed by 'right'?
        /// If yes, return `yes`. If not, return `no`
        fn followed_by<'src>(
            s: &mut Lexer<'src>,
            next: Option<char>,
            left: char,
            right: char,
            yes: TokenKind<'src>,
            no: TokenKind<'src>,
        ) -> Token<'src> {
            let byte_start = s.byte_pos;
            match next {
                Some(n) if n == right => {
                    s.byte_pos += left.len_utf8() + right.len_utf8();
                    Token {
                        kind: yes,
                        byte_start,
                    }
                }
                _ => {
                    s.byte_pos += left.len_utf8();
                    Token {
                        kind: no,
                        byte_start,
                    }
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
                '/' if {
                    let n = cs.next();
                    n == Some('/') || n == Some('*')
                } =>
                {
                    todo!("comments are not yet handled!")
                }
                '(' => break Some(ret_and_adv(self, '(', TK::ParenOpen)),
                ')' => break Some(ret_and_adv(self, ')', TK::ParenClose)),
                '{' => break Some(ret_and_adv(self, '{', TK::BraceOpen)),
                '}' => break Some(ret_and_adv(self, '}', TK::BraceClose)),
                '[' => break Some(ret_and_adv(self, '[', TK::BracketOpen)),
                ']' => break Some(ret_and_adv(self, ']', TK::BracketClose)),
                ',' => break Some(ret_and_adv(self, ',', TK::Comma)),
                ':' => break Some(ret_and_adv(self, ':', TK::Colon)),
                ';' => break Some(ret_and_adv(self, ';', TK::Semicolon)),
                '.' => break Some(ret_and_adv(self, '.', TK::Dot)),
                '+' => break f(self, cs.next(), '+', '=', TK::PlusEqual, TK::Plus),
                // TODO: also allow negative numbers
                '-' => break f(self, cs.next(), '-', '=', TK::MinusEqual, TK::Minus),
                '*' => break f(self, cs.next(), '*', '=', TK::StarEqual, TK::Star),
                '/' => break f(self, cs.next(), '/', '=', TK::StarEqual, TK::Star),
                '%' => break f(self, cs.next(), '%', '=', TK::PercentEqual, TK::Percent),
                '~' => break f(self, cs.next(), '~', '=', TK::TildeEqual, TK::Tilde),
                '!' => break f(self, cs.next(), '!', '=', TK::BangEqual, TK::Bang),
                '=' => break f(self, cs.next(), '=', '=', TK::EqualEqual, TK::Equal),
                '<' => break f(self, cs.next(), '<', '=', TK::LessEqual, TK::Less),
                '>' => break f(self, cs.next(), '>', '=', TK::GreaterEqual, TK::Greater),
                '&' => break f(self, cs.next(), '&', '&', TK::DoubleAmpersnd, TK::Ampersand),
                '|' => break f(self, cs.next(), '|', '|', TK::DoublePipe, TK::Pipe),
                '"' => todo!("strings are not yet handled"),
                c if c.is_digit(10) => {
                    todo!("cannot parse numbers yet")
                }
                c if c == '_' || c.is_alphabetic() => {
                    let byte_start = self.byte_pos;
                    let mut end = byte_start + c.len_utf8();
                    while let Some(next) = cs.next()
                        && (next.is_alphanumeric() || next == '_')
                    {
                        end += next.len_utf8()
                    }
                    self.byte_pos = end;

                    let ident = &self.whole[byte_start..end];
                    break Some(if let Ok(keyword) = Keyword::from_str(ident) {
                        Token {
                            kind: TokenKind::Keyword(keyword),
                            byte_start,
                        }
                    } else {
                        Token {
                            kind: TokenKind::Ident(ident),
                            byte_start,
                        }
                    });
                }
                _ => break None,
            }
        }
    }
}

#[test]
fn basic() {
    let s = "x && y;";
    let mut l = Lexer::new(s);
    assert_eq!(TokenKind::Ident("x"), l.next().unwrap().kind);
    assert_eq!(TokenKind::DoubleAmpersnd, l.next().unwrap().kind);
    assert_eq!(TokenKind::Ident("y"), l.next().unwrap().kind);
    assert_eq!(TokenKind::Semicolon, l.next().unwrap().kind);
    assert_eq!(None, l.next());
}

#[test]
fn multichar_ident() {
    let s = "hola && adeu || (si % no)";
    let mut l = Lexer::new(s);
    assert_eq!(TokenKind::Ident("hola"), l.next().unwrap().kind);
    assert_eq!(TokenKind::DoubleAmpersnd, l.next().unwrap().kind);
    assert_eq!(TokenKind::Ident("adeu"), l.next().unwrap().kind);
    assert_eq!(TokenKind::DoublePipe, l.next().unwrap().kind);

    assert_eq!(TokenKind::ParenOpen, l.next().unwrap().kind);
    assert_eq!(TokenKind::Ident("si"), l.next().unwrap().kind);
    assert_eq!(TokenKind::Percent, l.next().unwrap().kind);
    assert_eq!(TokenKind::Ident("no"), l.next().unwrap().kind);
    assert_eq!(TokenKind::ParenClose, l.next().unwrap().kind);
    assert_eq!(None, l.next());
}

#[test]
fn function_def() {
    let s = "void main() {}";
    let mut l = Lexer::new(s);
    assert_eq!(TokenKind::Keyword(Keyword::KwVoid), l.next().unwrap().kind);
    assert_eq!(TokenKind::Ident("main"), l.next().unwrap().kind);

    assert_eq!(TokenKind::ParenOpen, l.next().unwrap().kind);
    assert_eq!(TokenKind::ParenClose, l.next().unwrap().kind);

    assert_eq!(TokenKind::BraceOpen, l.next().unwrap().kind);
    assert_eq!(TokenKind::BraceClose, l.next().unwrap().kind);
}

#[test]
fn function_def_with_args() {
    use Keyword as Kw;
    use TokenKind as TK;
    let s = "void main(int argc, char **argv) {}";
    let mut l = Lexer::new(s);

    assert_eq!(TK::Keyword(Keyword::KwVoid), l.next().unwrap().kind);
    assert_eq!(TK::Ident("main"), l.next().unwrap().kind);

    assert_eq!(TK::ParenOpen, l.next().unwrap().kind);

    assert_eq!(TK::Keyword(Kw::KwInt), l.next().unwrap().kind);
    assert_eq!(TK::Ident("argc"), l.next().unwrap().kind);
    assert_eq!(TK::Comma, l.next().unwrap().kind);
    assert_eq!(TK::Keyword(Kw::KwChar), l.next().unwrap().kind);
    assert_eq!(TK::Star, l.next().unwrap().kind);
    assert_eq!(TK::Star, l.next().unwrap().kind);
    assert_eq!(TK::Ident("argv"), l.next().unwrap().kind);

    assert_eq!(TK::ParenClose, l.next().unwrap().kind);

    assert_eq!(TK::BraceOpen, l.next().unwrap().kind);
    assert_eq!(TK::BraceClose, l.next().unwrap().kind);
}

#[test]
fn function_def_with_args_and_newlines() {
    use Keyword as Kw;
    use TokenKind as TK;
    let s = "void\nmain(int argc,\nchar **argv)\n{\n\n}\n\n\n";
    let mut l = Lexer::new(s);

    assert_eq!(TK::Keyword(Keyword::KwVoid), l.next().unwrap().kind);
    assert_eq!(TK::Ident("main"), l.next().unwrap().kind);

    assert_eq!(TK::ParenOpen, l.next().unwrap().kind);

    assert_eq!(TK::Keyword(Kw::KwInt), l.next().unwrap().kind);
    assert_eq!(TK::Ident("argc"), l.next().unwrap().kind);
    assert_eq!(TK::Comma, l.next().unwrap().kind);
    assert_eq!(TK::Keyword(Kw::KwChar), l.next().unwrap().kind);
    assert_eq!(TK::Star, l.next().unwrap().kind);
    assert_eq!(TK::Star, l.next().unwrap().kind);
    assert_eq!(TK::Ident("argv"), l.next().unwrap().kind);

    assert_eq!(TK::ParenClose, l.next().unwrap().kind);

    assert_eq!(TK::BraceOpen, l.next().unwrap().kind);
    assert_eq!(TK::BraceClose, l.next().unwrap().kind);
}
