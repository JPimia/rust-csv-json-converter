use clap::error::Result;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::{fs, io};
use std::fs::{File, OpenOptions};
use clap::{Parser, Subcommand};
use csv::Reader;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    User {
        #[command(subcommand)]
        action: UserAction
    },
    Convert {
        input: String,
        output: String
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
    email: String
}

#[derive(Subcommand, Debug)]
enum UserAction {
    Create {
        name: String,
        age: u32,
        email: String
    }
}

fn csv_to_json(input_path: String, output_path: String) -> io::Result<()> {
    if fs::metadata(&input_path)?.len() == u64::MIN {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Input CSV file is empty."))
    }

    let mut reader = Reader::from_path(input_path)?;
    let headers = reader.headers()?;

    let required_columns = vec!["name", "age", "email"];
    for &column in required_columns.iter() {
        if !headers.iter().any(|col| col == column) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("CSV is missing column: {}", column)));
        }
    }
    let records: Vec<Person> = reader.deserialize().collect::<Result<_,_>>()?;

    let json_file = File::create(output_path)?;
    let buf_writer = BufWriter::new(json_file);
    serde_json::to_writer_pretty(buf_writer, &records)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::User { action } => match action {
            UserAction::Create { name, age, email } => {
                let new_user = Person { name, age, email };
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open("output.json")?;

                let mut users: Vec<Person> = if file.metadata()?.len() > u64::MIN {
                    let reader = BufReader::new(file);
                    serde_json::from_reader(reader)?
                } else {
                    Vec::new()
                };

                users.push(new_user);
                let writer = BufWriter::new(File::create("output.json")?);
                serde_json::to_writer_pretty(writer, &users)?;
            }
        },
        Commands::Convert { input, output } => {
            csv_to_json(input, output)?;
        }
    }
    Ok(())
}
