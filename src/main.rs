use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

static FNAME : &str = "todo.json"; 

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
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let exe = env::current_exe()?;
    let exe_dir = exe.parent().unwrap();
    let path = exe_dir.join(FNAME);

    if path.exists() {
        println!("File exists: {}", path.display());
    } else {
        println!("File not found {}", path.display());
    }

    match cli.command {
        Commands::Create { name } => println!("Adding {}", name),
        Commands::Delete { id } => println!("Removing {}", id),
        Commands::List => println!("Listing items {}", FNAME),
    }

    Ok(())

}