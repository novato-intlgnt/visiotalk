//! Compiler debug tracer — prints each pipeline phase with visual formatting.
//! Only active when `--debug` flag is set; zero overhead otherwise.

use vt_core::{Attribute, Comparator, DetectRule, ModeDecl, Program, Unit};
use vt_lexer::Token;

pub struct CompilerDebugger;

impl CompilerDebugger {
    pub fn new() -> Self {
        Self
    }

    // ── Phase 1: Source ────────────────────────────────────────────

    pub fn print_source(&self, source: &str, filepath: &str) {
        println!("=== FASE 1: CÓDIGO FUENTE ({}) ===", filepath);
        println!("{}", source);
        println!();
    }

    // ── Phase 2: Tokens ────────────────────────────────────────────

    pub fn print_tokens(&self, tokens: &[(usize, Token, usize)]) {
        println!("=== FASE 2: TOKENS ({} tokens) ===", tokens.len());
        for (i, (_, token, _)) in tokens.iter().enumerate() {
            println!("{:>3}: {:<16} {}", i + 1, token_variant_name(token), token);
        }
        println!();
    }

    // ── Phase 3: AST ───────────────────────────────────────────────

    pub fn print_ast(&self, program: &Program) {
        println!("=== FASE 3: AST ===");
        println!("{}", ast_display(program));
    }
}

// ── Token helpers ──────────────────────────────────────────────────

/// Returns the Rust enum variant name for a Token, e.g. `"KwModo"`, `"Ident"`.
fn token_variant_name(token: &Token) -> &'static str {
    match token {
        Token::KwModo => "KwModo",
        Token::KwDetectar => "KwDetectar",
        Token::LBrace => "LBrace",
        Token::RBrace => "RBrace",
        Token::Colon => "Colon",
        Token::Le => "Le",
        Token::Ge => "Ge",
        Token::Eq => "Eq",
        Token::Lt => "Lt",
        Token::Gt => "Gt",
        Token::UnitCentimeters => "UnitCentimeters",
        Token::UnitFeet => "UnitFeet",
        Token::UnitPixels => "UnitPixels",
        Token::UnitMeters => "UnitMeters",
        Token::NumberLit(_) => "NumberLit",
        Token::Ident(_) => "Ident",
        Token::StringLit(_) => "StringLit",
    }
}

// ── AST tree pretty-printer ────────────────────────────────────────

fn ast_display(program: &Program) -> String {
    let mut out = String::from("Program\n");
    let children = count_children(program);

    let mut idx = 0;
    if let Some(ref mode) = program.mode {
        let is_last = idx == children - 1;
        format_mode(&mut out, "", is_last, mode);
        idx += 1;
    }

    let rule_count = program.rules.len();
    for (i, rule) in program.rules.iter().enumerate() {
        let is_last = idx == children - 1;
        format_rule(&mut out, "", is_last, i, rule_count, rule);
        idx += 1;
    }

    out
}

fn count_children(program: &Program) -> usize {
    (if program.mode.is_some() { 1 } else { 0 }) + program.rules.len()
}

fn format_mode(out: &mut String, prefix: &str, is_last: bool, mode: &ModeDecl) {
    let connector = if is_last { "└── " } else { "├── " };
    out.push_str(prefix);
    out.push_str(connector);
    out.push_str(&format!("ModeDecl: \"{}\"\n", mode.name));
}

fn format_rule(
    out: &mut String,
    prefix: &str,
    is_last: bool,
    index: usize,
    total: usize,
    rule: &DetectRule,
) {
    let connector = if is_last { "└── " } else { "├── " };
    out.push_str(prefix);
    out.push_str(connector);
    out.push_str(&format!("Rule[{}]: DETECTAR \"{}\"\n", index, rule.label));

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    let attr_count = rule.attributes.len();
    for (i, attr) in rule.attributes.iter().enumerate() {
        let last_attr = i == attr_count - 1;
        format_attribute(out, &child_prefix, last_attr, attr);
    }

    // If there are more rules after this one, the continuation line
    // between rules uses the same prefix as the rule itself.
    if !is_last && index + 1 < total {
        let divider = format!("{}{}", prefix, "│   \n");
        if !out.ends_with(&divider) && index + 1 < total {
            // Only add divider if there are attributes (already handled by child_prefix)
            if rule.attributes.is_empty() {
                out.push_str(&format!("{}│   \n", prefix));
            }
        }
    }
}

fn format_attribute(out: &mut String, prefix: &str, is_last: bool, attr: &Attribute) {
    let connector = if is_last { "└── " } else { "├── " };
    out.push_str(prefix);
    out.push_str(connector);

    match attr {
        Attribute::Distance(expr) => {
            out.push_str(&format!(
                "Distance: {} {} {}\n",
                comparator_str(&expr.comparator),
                format_value(expr.value),
                unit_str(&expr.unit),
            ));
        }
        Attribute::Alert(s) => {
            out.push_str(&format!("Alert: \"{}\"\n", s));
        }
        Attribute::Priority(n) => {
            out.push_str(&format!("Priority: {}\n", n));
        }
    }
}

fn comparator_str(c: &Comparator) -> &'static str {
    match c {
        Comparator::Lt => "<",
        Comparator::Gt => ">",
        Comparator::Le => "<=",
        Comparator::Ge => ">=",
        Comparator::Eq => "==",
    }
}

fn unit_str(u: &Unit) -> &'static str {
    match u {
        Unit::Meters => "m",
        Unit::Centimeters => "cm",
        Unit::Feet => "ft",
        Unit::Pixels => "px",
    }
}

/// Formats f64 values: integers without decimal, floats with one decimal place.
fn format_value(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.0}", v)
    } else {
        format!("{:.1}", v)
    }
}
