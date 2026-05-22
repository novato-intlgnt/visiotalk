//! Parser for the VisioTalk DSL.
//! The grammar is defined in `src/grammar.lalrpop` and compiled by LALRPOP.

#![allow(clippy::empty_line_after_outer_attr)]

use lalrpop_util::lalrpop_mod;
use vt_lexer::tokenize_with_locations;

lalrpop_mod!(pub grammar);

/// A structured parse error with location information.
#[derive(Debug, Clone, PartialEq)]
pub struct VtParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

/// Parses a VisioTalk source string into an AST.
///
/// Returns `Ok(Program)` on success, or `Vec<VtParseError>` with all errors found.
pub fn parse(source: &str) -> Result<vt_core::Program, Vec<VtParseError>> {
    let tokens = tokenize_with_locations(source);

    let program = grammar::ProgramParser::new().parse(tokens).map_err(|err| {
        let (line, column) = match &err {
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (loc, _, _), ..
            }
            | lalrpop_util::ParseError::InvalidToken { location: loc }
            | lalrpop_util::ParseError::ExtraToken { token: (loc, _, _) } => {
                let line = source[..*loc].matches('\n').count() + 1;
                let last_nl = source[..*loc].rfind('\n').map(|p| p + 1).unwrap_or(0);
                (line, loc - last_nl + 1)
            }
            lalrpop_util::ParseError::User { .. }
            | lalrpop_util::ParseError::UnrecognizedEof { .. } => (0, 0),
        };

        vec![VtParseError {
            message: format!("{:?}", err),
            line,
            column,
        }]
    })?;

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert!(parse("").is_err());
    }
}
