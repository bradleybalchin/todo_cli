use clap::{Parser, Subcommand};
use core::fmt;
use std::{env, path::PathBuf};
use serde::{Deserialize, Serialize};

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
        name: Vec<String>,
    },
    Delete {
        id: u32,
    },
    List,
    Reset,
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
        writeln!(f, "------- TODO -------")?;
        for item in &self.0 {
            writeln!(f, "[{}] {}", item.id, item.name)?;
        }
        Ok(())
        
    }
    
}


// initialise todo.json if does not exist
fn init_list(path : &PathBuf) -> std::io::Result<()> {
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);

    let items:TodoList = TodoList(vec![]);
    serde_json::to_writer_pretty(writer, &items)?;

    Ok(())
}

// create a new item and add to list
fn create(name : String, items : &mut TodoList) {
    let id  = items.0.len().try_into().unwrap_or(0) + 1;
    items.0.push(Item { id, name });
}

// delete item from the list
fn delete(id : u32, items : &mut TodoList) {
    items.0.retain(|item| item.id != id);

    for (index, item) in items.0.iter_mut().enumerate() {
        item.id = (index + 1) as u32;
    } 

}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let exe = env::current_exe()?;
    let exe_dir = exe.parent().unwrap();
    let path = exe_dir.join(FNAME);

    // empty items list, load in new list from todo.json if exists
    let mut items:TodoList = TodoList(vec![]);

    if path.exists() {
        println!("File exists: {}", path.display());
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        items = serde_json::from_reader(reader)?;
    } else {
        println!("File not found {}, creating todo.json", path.display());
        init_list(&path)?;
    }

    // Perform Command
    match cli.command {
        Commands::Create { name } => {
            let name = name.join(" ");
            println!("Adding {}", name);
            create(name, &mut items)
        }  ,
        Commands::Delete { id } => {
            println!("Removing task {}", id);
            delete(id, &mut items)
        },
        Commands::List => println!("{}", items),
        Commands::Reset => {
            println!("Resetting todo.json");
            items = TodoList(vec![]);
        }
    }

    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &items)?;

    Ok(())

}