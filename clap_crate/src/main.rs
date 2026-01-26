use clap::{Args, Parser, Subcommand, ValueEnum, command};

#[derive(Parser)]
#[command(name = "SuperclAppi")]
#[command(author = "SclAi's Author")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a file
    Add {
        /// File path to add
        #[arg(required = true)]
        path: String,

        /// Priority level
        #[arg(short, long, value_enum, default_value_t = Priority::Low)]
        priority: Priority,
    },

    /// Drops a file
    Drop { name: String },

    /// Group example
    Network(NetworkArgs),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Priority {
    Low,
    High,
    Critical,
}

#[derive(Args)]
struct NetworkArgs {
    #[arg(long)]
    ip: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode ON");
    }

    match &cli.command {
        Commands::Add { path, priority } => {
            println!("Adding {} with {:?} priority", path, priority);
        }
        Commands::Drop { name } => {
            println!("Dropping {}", name);
        }
        Commands::Network(args) => {
            println!("Connecting to {}:{}", args.ip, args.port);
        }
    }
}
