use std::path::Path;

fn main() {
    let grammar_file = Path::new("src/grammar.lalrpop");
    if grammar_file.exists() {
        lalrpop::process_root().unwrap();
    } else {
        println!("cargo:warning=grammar.lalrpop not found — skipping parser generation");
    }
}
