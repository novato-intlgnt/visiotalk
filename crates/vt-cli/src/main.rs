use clap::{Parser, Subcommand};

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
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Compile {
            input,
            target: _target,
        } => {
            let source = match std::fs::read_to_string(&input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read '{}': {}", input, e);
                    std::process::exit(1);
                }
            };

            match vt_parser::parse(&source) {
                Ok(program) => {
                    println!("{:#?}", program);
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
