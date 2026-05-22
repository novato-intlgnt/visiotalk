use clap::Parser;

#[derive(Parser)]
#[command(name = "vt")]
#[command(about = "VisioTalk compiler — transfiles .vt DSL files to executable scripts")]
struct Cli {
    #[arg(value_name = "INPUT")]
    input: String,

    #[arg(short, long, value_name = "TARGET", default_value = "python")]
    target: String,
}

fn main() {
    let cli = Cli::parse();
    println!("VisioTalk compiler v0.1.0");
    println!("  Input: {}", cli.input);
    println!("  Target: {}", cli.target);
    println!("  (parser & codegen not yet integrated)");
}
