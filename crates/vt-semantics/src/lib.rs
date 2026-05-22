use vt_core::Program;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub fn analyze(program: &Program) -> Result<(), Vec<SemanticError>> {
    let mut errors = Vec::new();

    if program.rules.is_empty() {
        errors.push(SemanticError {
            message: "El programa debe contener al menos una regla DETECTAR".into(),
            line: 0,
            column: 0,
        });
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vt_core::Program;

    #[test]
    fn empty_rules_is_error() {
        let program = Program {
            mode: None,
            rules: vec![],
        };
        assert!(analyze(&program).is_err());
    }
}
