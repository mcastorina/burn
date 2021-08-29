use logos::Logos;

#[derive(Debug, PartialEq, Logos, Copy, Clone)]
pub enum Token {
    #[error]
    Error,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    // Whitespace,
    #[regex("[a-zA-Z][a-zA-Z0-9_]*")]
    Ident,
    #[regex("[0-9_]+", |lex| lex.slice().replace('_', "").parse())]
    Number(usize),
    #[regex(r#""([^"]|\.)*"|'([^']|\.)*'"#)]
    String,
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
    #[token("->")]
    RArrow,
    #[token("&")]
    Ampersand,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("=")]
    Assign,
    #[token(":=")]
    Declare,
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("/=")]
    DivideEqual,
    #[token("*=")]
    MultiplyEqual,
    #[token("^=")]
    XorEqual,
    #[token("%=")]
    PercentEqual,
    #[token("|=")]
    OrEqual,
    #[token("&=")]
    AndEqual,
    #[token(">>=")]
    RightShiftEqual,
    #[token("<<=")]
    LeftShiftEqual,
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
    #[regex("//[^\n]*")]
    Comment,
    #[token(".")]
    Period,
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
    #[token("import")]
    KeywordImport,
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
    #[token("yield")]
    KeywordYield,

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
            Token::RArrow => write!(f, "->"),
            Token::Ampersand => write!(f, "&"),
            Token::LeftShift => write!(f, "<<"),
            Token::RightShift => write!(f, ">>"),
            Token::Assign => write!(f, "="),
            Token::Declare => write!(f, ":="),
            Token::PlusEqual => write!(f, "+="),
            Token::MinusEqual => write!(f, "-="),
            Token::DivideEqual => write!(f, "/="),
            Token::MultiplyEqual => write!(f, "*="),
            Token::XorEqual => write!(f, "^="),
            Token::PercentEqual => write!(f, "%="),
            Token::OrEqual => write!(f, "|="),
            Token::AndEqual => write!(f, "&="),
            Token::RightShiftEqual => write!(f, ">>="),
            Token::LeftShiftEqual => write!(f, "<<="),
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
            Token::Period => write!(f, "."),
            Token::KeywordBreak => write!(f, "break"),
            Token::KeywordContinue => write!(f, "continue"),
            Token::KeywordElse => write!(f, "else"),
            Token::KeywordFalse => write!(f, "false"),
            Token::KeywordFor => write!(f, "for"),
            Token::KeywordFn => write!(f, "fn"),
            Token::KeywordIf => write!(f, "if"),
            Token::KeywordImport => write!(f, "import"),
            Token::KeywordIn => write!(f, "in"),
            Token::KeywordNone => write!(f, "none"),
            Token::KeywordReturn => write!(f, "return"),
            Token::KeywordTrue => write!(f, "true"),
            Token::KeywordWhile => write!(f, "while"),
            Token::KeywordYield => write!(f, "yield"),
            Token::EOF => write!(f, "<EOF>"),
        }
    }
}

#[test]
fn display() {
    assert_eq!(Token::LeftCurlyBracket.to_string(), "{");
    assert_eq!(Token::RightCurlyBracket.to_string(), "}");
    assert_eq!(Token::Plus.to_string(), "+");
}
