use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI for todo management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },
    List,
    Exit,
}

fn main() {
    let cli = Cli::parse();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    match &cli.command {
        Commands::Add { text } => {
            print!("Added new task {}", String::from(text) + "\n");
            file.write_all(format!("{}\n", text).as_bytes()).unwrap();
        }

        Commands::List => {
            let contents = fs::read_to_string("todo.txt").expect("File probably doesn't exist!");
            println!("{}", contents)
        }

        Commands::Exit => {
            println!("Exiting...");
        }
    }
}
