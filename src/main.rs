mod database;

use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use uuid::Uuid;
use database::{connection, task_dao};
use database::task::Task;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    Remove { id: String },
    Show,
    Done { id: String },
    Undone { id: String }
}

fn parse_id(value: &String) -> Uuid {
    Uuid::parse_str(value.as_str()).expect("Incorrect id format!")
}

fn show(connection: &Connection) -> Result<()>{
    let mut statement = task_dao::get_all(connection).expect("Unexpected error!");
    statement.query_map([], Task::parse_row).map(|tasks| {
        tasks.for_each(|task| println!("{}", task.expect("Error detected while parsing row!")))
    })
}

fn main() {

    let cli = Cli::parse();
    let db_connection = connection::establish();

    match &cli.command {
        Commands::Add { description } => {
            task_dao::insert(&db_connection, Task::new(description.clone())).and_then(|_| show(&db_connection))
        }
        Commands::Remove {id } => {
            task_dao::delete(&db_connection, parse_id(id)).and_then(|_| show(&db_connection))
        }
        Commands::Show => {
            show(&db_connection)
        }
        Commands::Done {id} => {
            task_dao::mark(&db_connection, parse_id(id), true).and_then(|_| show(&db_connection))
        }
        Commands::Undone {id} => {
            task_dao::mark(&db_connection, parse_id(id), false ).and_then(|_| show(&db_connection))
        }
    }.expect("Unexpected error!")
}
