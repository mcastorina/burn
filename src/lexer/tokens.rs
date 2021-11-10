use logos::Logos;

#[derive(Debug, PartialEq, Logos, Copy, Clone)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    // Whitespace,
    #[error]
    Error,

    #[regex("[a-zA-Z][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9_]+", |lex| lex.slice().replace('_', "").parse())]
    Number(usize),
    #[regex(r#""([^"]|\.)*"|'([^']|\.)*'"#)]
    String,
    #[regex(r#"`([^\\]|\.)`"#)]
    Byte,
    #[token("_")]
    Underscore,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("|")]
    Pipe,
    #[token("++")]
    Inc,
    #[token("--")]
    Dec,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Bang,
    #[token("~")]
    Tilde,
    #[token("?")]
    Question,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    RightArrow,
    #[token("&")]
    Ampersand,
    #[token("=")]
    Assign,
    #[token(":=")]
    Declare,
    #[token("{")]
    LeftCurlyBracket,
    #[token("}")]
    RightCurlyBracket,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftSquarBracket,
    #[token("]")]
    RightSquareBracket,
    #[token("<")]
    LeftAngleBracket,
    #[token(">")]
    RightAngleBracket,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token(">=")]
    GreaterOrEqual,
    #[token("<=")]
    LessOrEqual,
    #[regex("//[^\n]*", logos::skip)]
    Comment,
    #[token(".")]
    Dot,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,
    #[token("else")]
    KeywordElse,
    #[token("false")]
    KeywordFalse,
    #[token("for")]
    KeywordFor,
    #[token("fn")]
    KeywordFn,
    #[token("if")]
    KeywordIf,
    #[token("in")]
    KeywordIn,
    #[token("none")]
    KeywordNone,
    #[token("return")]
    KeywordReturn,
    #[token("true")]
    KeywordTrue,
    #[token("while")]
    KeywordWhile,

    EOF,
}

use std::fmt::{Display, Error, Formatter};
impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Error => write!(f, "error"),
            // Token::Whitespace => write!(f, "<ws>"),
            Token::Ident => write!(f, "ident"),
            Token::Number(n) => write!(f, "num({})", n),
            Token::String => write!(f, "string"),
            Token::Byte => write!(f, "byte"),
            Token::Underscore => write!(f, "_"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::Pipe => write!(f, "|"),
            Token::Inc => write!(f, "++"),
            Token::Dec => write!(f, "--"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Bang => write!(f, "!"),
            Token::Tilde => write!(f, "~"),
            Token::Question => write!(f, "?"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::DoubleColon => write!(f, "::"),
            Token::RightArrow => write!(f, "->"),
            Token::Ampersand => write!(f, "&"),
            Token::Assign => write!(f, "="),
            Token::Declare => write!(f, ":="),
            Token::LeftCurlyBracket => write!(f, "{{"),
            Token::RightCurlyBracket => write!(f, "}}"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftSquarBracket => write!(f, "["),
            Token::RightSquareBracket => write!(f, "]"),
            Token::LeftAngleBracket => write!(f, "<"),
            Token::RightAngleBracket => write!(f, ">"),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::GreaterOrEqual => write!(f, ">="),
            Token::LessOrEqual => write!(f, "<="),
            Token::Comment => write!(f, "// comment"),
            Token::Dot => write!(f, "."),
            Token::KeywordBreak => write!(f, "break"),
            Token::KeywordContinue => write!(f, "continue"),
            Token::KeywordElse => write!(f, "else"),
            Token::KeywordFalse => write!(f, "false"),
            Token::KeywordFor => write!(f, "for"),
            Token::KeywordFn => write!(f, "fn"),
            Token::KeywordIf => write!(f, "if"),
            Token::KeywordIn => write!(f, "in"),
            Token::KeywordNone => write!(f, "none"),
            Token::KeywordReturn => write!(f, "return"),
            Token::KeywordTrue => write!(f, "true"),
            Token::KeywordWhile => write!(f, "while"),
            Token::EOF => write!(f, "<EOF>"),
        }
    }
}

#[macro_export]
macro_rules! T {
    [error] => { $crate::lexer::Token::Error };
    // [ws] => { $crate::lexer::Token::Whitespace };
    [ident] => { $crate::lexer::Token::Ident };
    [num($i:pat)] => { $crate::lexer::Token::Number($i) };
    [string] => { $crate::lexer::Token::String };
    [byte] => { $crate::lexer::Token::Byte };
    [_] => { $crate::lexer::Token::Underscore };
    [+] => { $crate::lexer::Token::Plus };
    [-] => { $crate::lexer::Token::Minus };
    [*] => { $crate::lexer::Token::Asterisk };
    [/] => { $crate::lexer::Token::Slash };
    [%] => { $crate::lexer::Token::Percent };
    [^] => { $crate::lexer::Token::Caret };
    [|] => { $crate::lexer::Token::Pipe };
    [++] => { $crate::lexer::Token::Inc };
    [--] => { $crate::lexer::Token::Dec };
    [&&] => { $crate::lexer::Token::And };
    [||] => { $crate::lexer::Token::Or };
    [!] => { $crate::lexer::Token::Bang };
    [~] => { $crate::lexer::Token::Tilde };
    [?] => { $crate::lexer::Token::Question };
    [,] => { $crate::lexer::Token::Comma };
    [;] => { $crate::lexer::Token::Semicolon };
    [:] => { $crate::lexer::Token::Colon };
    [::] => { $crate::lexer::Token::DoubleColon };
    [->] => { $crate::lexer::Token::RightArrow };
    [&] => { $crate::lexer::Token::Ampersand };
    [=] => { $crate::lexer::Token::Assign };
    [:=] => { $crate::lexer::Token::Declare };
    ['{'] => { $crate::lexer::Token::LeftCurlyBracket };
    ['}'] => { $crate::lexer::Token::RightCurlyBracket };
    ['('] => { $crate::lexer::Token::LeftParen };
    [')'] => { $crate::lexer::Token::RightParen };
    ['['] => { $crate::lexer::Token::LeftSquarBracket };
    [']'] => { $crate::lexer::Token::RightSquareBracket };
    [<] => { $crate::lexer::Token::LeftAngleBracket };
    [>] => { $crate::lexer::Token::RightAngleBracket };
    [==] => { $crate::lexer::Token::Equal };
    [!=] => { $crate::lexer::Token::NotEqual };
    [>=] => { $crate::lexer::Token::GreaterOrEqual };
    [<=] => { $crate::lexer::Token::LessOrEqual };
    [comment] => { $crate::lexer::Token::Comment };
    [.] => { $crate::lexer::Token::Dot };
    [break] => { $crate::lexer::Token::KeywordBreak };
    [continue] => { $crate::lexer::Token::KeywordContinue };
    [else] => { $crate::lexer::Token::KeywordElse };
    [false] => { $crate::lexer::Token::KeywordFalse };
    [for] => { $crate::lexer::Token::KeywordFor };
    [fn] => { $crate::lexer::Token::KeywordFn };
    [if] => { $crate::lexer::Token::KeywordIf };
    [in] => { $crate::lexer::Token::KeywordIn };
    [none] => { $crate::lexer::Token::KeywordNone };
    [return] => { $crate::lexer::Token::KeywordReturn };
    [true] => { $crate::lexer::Token::KeywordTrue };
    [while] => { $crate::lexer::Token::KeywordWhile };
    [eof] => { $crate::lexer::Token::EOF };
}

#[test]
fn display() {
    assert_eq!(Token::LeftCurlyBracket.to_string(), "{");
    assert_eq!(Token::RightCurlyBracket.to_string(), "}");
    assert_eq!(Token::Plus.to_string(), "+");
}
