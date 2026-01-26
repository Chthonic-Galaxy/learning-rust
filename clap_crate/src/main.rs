use clap::{Args, Parser, Subcommand, ValueEnum, command};

#[derive(Parser)]
#[command(name = "filetool")]
struct Cli {
    #[arg(required = true)]
    input: String,

    #[arg(short, long, value_name = "OUTPUT_PATH")]
    output: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Mode::Safe)]
    mode: Mode,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints info
    Info,

    /// Convert to <format>
    Convert { format: String },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Fast,
    Safe,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info => {
            println!("Getting info for {}", cli.input)
        }
        Commands::Convert { format } => {
            println!("Converted to {} format", format)
        }
    }
}
