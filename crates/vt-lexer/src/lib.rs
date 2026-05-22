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

pub fn tokenize(source: &str) -> Vec<Token> {
    Token::lexer(source).flatten().collect()
}

pub fn tokenize_with_locations(source: &str) -> Vec<(usize, Token, usize)> {
    Token::lexer(source)
        .spanned()
        .filter_map(|(result, span)| result.ok().map(|t| (span.start, t, span.end)))
        .collect()
}

use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::KwModo => write!(f, "MODO"),
            Token::KwDetectar => write!(f, "DETECTAR"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Colon => write!(f, ":"),
            Token::Le => write!(f, "<="),
            Token::Ge => write!(f, ">="),
            Token::Eq => write!(f, "=="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::UnitCentimeters => write!(f, "cm"),
            Token::UnitFeet => write!(f, "ft"),
            Token::UnitPixels => write!(f, "px"),
            Token::UnitMeters => write!(f, "m"),
            Token::NumberLit(n) => write!(f, "{}", n),
            Token::Ident(s) => write!(f, "{}", s),
            Token::StringLit(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(source: &str) -> Vec<Token> {
        tokenize(source)
    }

    // --- 1.1 Keyword tokens ---

    #[test]
    fn keyword_modo() {
        assert_eq!(tokens("MODO"), vec![Token::KwModo]);
    }

    #[test]
    fn keyword_detectar() {
        assert_eq!(tokens("DETECTAR"), vec![Token::KwDetectar]);
    }

    #[test]
    fn keyword_priority_over_ident() {
        // "MODO" and "DETECTAR" are keywords, not identifiers
        let toks = tokens("MODO DETECTAR");
        assert_eq!(toks, vec![Token::KwModo, Token::KwDetectar]);
        // Verify they are NOT parsed as Ident
        for t in &toks {
            assert!(!matches!(t, Token::Ident(_)));
        }
    }

    // --- 1.2 Delimiter tokens ---

    #[test]
    fn delimiter_lbrace() {
        assert_eq!(tokens("{"), vec![Token::LBrace]);
    }

    #[test]
    fn delimiter_rbrace() {
        assert_eq!(tokens("}"), vec![Token::RBrace]);
    }

    #[test]
    fn delimiter_colon() {
        assert_eq!(tokens(":"), vec![Token::Colon]);
    }

    // --- 1.3 Comparator tokens ---

    #[test]
    fn comparator_lt() {
        assert_eq!(tokens("<"), vec![Token::Lt]);
    }

    #[test]
    fn comparator_gt() {
        assert_eq!(tokens(">"), vec![Token::Gt]);
    }

    #[test]
    fn comparator_le() {
        assert_eq!(tokens("<="), vec![Token::Le]);
    }

    #[test]
    fn comparator_ge() {
        assert_eq!(tokens(">="), vec![Token::Ge]);
    }

    #[test]
    fn comparator_eq() {
        assert_eq!(tokens("=="), vec![Token::Eq]);
    }

    #[test]
    fn all_comparators_sequence() {
        let toks = tokens("< > <= >= ==");
        assert_eq!(
            toks,
            vec![Token::Lt, Token::Gt, Token::Le, Token::Ge, Token::Eq]
        );
    }

    // --- 1.4 Unit tokens (not swallowed by Ident) ---

    #[test]
    fn unit_meters() {
        assert_eq!(tokens("m"), vec![Token::UnitMeters]);
        assert!(!matches!(tokens("m")[0], Token::Ident(_)));
    }

    #[test]
    fn unit_centimeters() {
        assert_eq!(tokens("cm"), vec![Token::UnitCentimeters]);
        assert!(!matches!(tokens("cm")[0], Token::Ident(_)));
    }

    #[test]
    fn unit_feet() {
        assert_eq!(tokens("ft"), vec![Token::UnitFeet]);
        assert!(!matches!(tokens("ft")[0], Token::Ident(_)));
    }

    #[test]
    fn unit_pixels() {
        assert_eq!(tokens("px"), vec![Token::UnitPixels]);
        assert!(!matches!(tokens("px")[0], Token::Ident(_)));
    }

    #[test]
    fn all_units_sequence() {
        assert_eq!(
            tokens("m cm ft px"),
            vec![
                Token::UnitMeters,
                Token::UnitCentimeters,
                Token::UnitFeet,
                Token::UnitPixels
            ]
        );
    }

    // --- 1.5 Literals ---

    #[test]
    fn number_lit_integer() {
        assert_eq!(tokens("42"), vec![Token::NumberLit(42.0)]);
    }

    #[test]
    fn number_lit_float() {
        assert_eq!(tokens("2.5"), vec![Token::NumberLit(2.5)]);
    }

    #[test]
    fn number_lit_zero() {
        assert_eq!(tokens("0"), vec![Token::NumberLit(0.0)]);
    }

    #[test]
    fn number_lit_leading_decimal() {
        assert_eq!(tokens("0.5"), vec![Token::NumberLit(0.5)]);
    }

    #[test]
    fn string_lit_content() {
        assert_eq!(
            tokens(r#""Alerta de prueba""#),
            vec![Token::StringLit("Alerta de prueba".to_string())]
        );
    }

    #[test]
    fn string_lit_empty() {
        assert_eq!(tokens(r#""""#), vec![Token::StringLit("".to_string())]);
    }

    // --- 1.6 Whitespace and comments ---

    #[test]
    fn whitespace_spaces() {
        // Spaces between tokens are skipped
        assert_eq!(
            tokens("MODO   DETECTAR"),
            vec![Token::KwModo, Token::KwDetectar]
        );
    }

    #[test]
    fn whitespace_tabs() {
        assert_eq!(
            tokens("MODO\t\tDETECTAR"),
            vec![Token::KwModo, Token::KwDetectar]
        );
    }

    #[test]
    fn whitespace_newlines() {
        assert_eq!(
            tokens("MODO\n\nDETECTAR"),
            vec![Token::KwModo, Token::KwDetectar]
        );
    }

    #[test]
    fn comment_single_line() {
        let toks = tokens("// este es un comentario\nMODO");
        assert_eq!(toks, vec![Token::KwModo]);
    }

    #[test]
    fn comment_end_of_line() {
        let toks = tokens("MODO // modo de navegacion\nDETECTAR");
        assert_eq!(toks, vec![Token::KwModo, Token::KwDetectar]);
    }

    #[test]
    fn comment_only_line() {
        let toks = tokens("// solo comentario");
        assert!(toks.is_empty());
    }

    // --- 1.7 Empty input ---

    #[test]
    fn empty_input() {
        let toks = tokens("");
        assert!(toks.is_empty());
    }
}
