use clap::{Parser, Subcommand};

mod debug;

use crate::debug::CompilerDebugger;

#[derive(Parser)]
#[command(name = "vt")]
#[command(about = "VisioTalk compiler — transcompiles .vt DSL files to executable scripts")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile {
        #[arg(value_name = "INPUT")]
        input: String,

        #[arg(short, long, value_name = "TARGET", default_value = "python")]
        target: String,

        #[arg(long, default_value_t = false)]
        debug: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compile {
            input,
            target: _target,
            debug,
        } => {
            let source = match std::fs::read_to_string(&input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read '{}': {}", input, e);
                    std::process::exit(1);
                }
            };

            let debugger = if debug {
                Some(CompilerDebugger::new())
            } else {
                None
            };

            // ── Phase 1: Source ──────────────────────────────
            if let Some(ref d) = debugger {
                d.print_source(&source, &input);
            }

            // ── Phase 2: Lex (tokens) ────────────────────────
            let tokens = vt_lexer::tokenize_with_locations(&source);
            if let Some(ref d) = debugger {
                d.print_tokens(&tokens);
            }

            // ── Phase 3: Parse (AST) ─────────────────────────
            // Future: Phase 4 = Semantic analysis
            // Future: Phase 5 = Codegen
            match vt_parser::parse(&source) {
                Ok(program) => {
                    if let Some(ref d) = debugger {
                        d.print_ast(&program);
                    } else {
                        println!("{:#?}", program);
                    }
                }
                Err(errors) => {
                    for err in &errors {
                        eprintln!(
                            "error at line {}:{} : {}",
                            err.line, err.column, err.message
                        );
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}
