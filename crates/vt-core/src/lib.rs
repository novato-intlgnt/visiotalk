//! VisioTalk AST - Abstract Syntax Tree definitions.
//! All compiler phases share these types.

use serde::{Deserialize, Serialize};

/// Represents a parsed VisioTalk program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub mode: Option<ModeDecl>,
    pub rules: Vec<DetectRule>,
}

/// A `MODO <name>` declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeDecl {
    pub name: String,
}

/// A `DETECTAR <label> { ... }` rule block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectRule {
    pub label: String,
    pub attributes: Vec<Attribute>,
}

/// A key-value attribute inside a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    Distance(DistanceExpr),
    Alert(String),
    Priority(u32),
}

/// A distance comparison expression (e.g. `< 2.0m`, `>= 0.5px`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceExpr {
    pub comparator: Comparator,
    pub value: f64,
    pub unit: Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Comparator {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Unit {
    Meters,
    Centimeters,
    Feet,
    Pixels,
}
