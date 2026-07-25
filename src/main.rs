use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        name: String,
    },
    Delete {
        id: u32,
    },
    List,
    Save {
        fname: String,
    },
    Load {
        fname: String
    },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => println!("Adding {}", name),
        Commands::Delete { id } => println!("Removing {}", id),
        Commands::Save { fname } => println!("Removing {}", fname),
        Commands::Load { fname } => println!("Removing {}", fname),
        Commands::List => println!("Listing items"),
    }
}