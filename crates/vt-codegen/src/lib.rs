pub mod javascript;
pub mod python;

use vt_core::Program;

pub enum Target {
    Python,
    JavaScript,
}

pub fn generate(program: &Program, target: Target) -> String {
    match target {
        Target::Python => python::generate(program),
        Target::JavaScript => javascript::generate(program),
    }
}
