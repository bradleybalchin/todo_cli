use clap::{Parser, Subcommand};
use core::fmt;
use std::ffi::os_str::Display;
use std::{env, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;

const FNAME : &str = "todo.json"; 

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

// single todolist item
#[derive(Serialize, Deserialize, Debug)]
struct Item{
    id: u32,
    name : String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList(Vec<Item>);
// custom display format for todolist
impl  fmt::Display for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            writeln!(f, "[{}] {}", item.id, item.name)?;
        }
        Ok(())
        
    }
    
}


// initialise todo.json
fn init_list(){

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

   

    // creating list, and saving to json example
    let items = vec![
        Item {
            id:1,
            name: "Lock Front Door".to_string()
        },
        Item {
            id:2,
            name: "Lock Back Door".to_string()
        },
        Item {
            id:3,
            name: "Make Dinner".to_string(),
        },
    ];

    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &items)?;

    // opening json example
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let items: TodoList = serde_json::from_reader(reader)?;
    println!("{}", items);

    Ok(())

}