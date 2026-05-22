//! Lexer for the VisioTalk DSL.
//! Uses `logos` to generate a DFA-based tokenizer.

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(skip r"//[^\n]*")]
pub enum Token {
    #[token("MODO")]
    KwModo,

    #[token("DETECTAR")]
    KwDetectar,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(":")]
    Colon,

    #[token("<=")]
    Le,

    #[token(">=")]
    Ge,

    #[token("==")]
    Eq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("cm")]
    UnitCentimeters,

    #[token("ft")]
    UnitFeet,

    #[token("px")]
    UnitPixels,

    #[token("m")]
    UnitMeters,

    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().ok(), priority = 2)]
    NumberLit(f64),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    StringLit(String),
}
