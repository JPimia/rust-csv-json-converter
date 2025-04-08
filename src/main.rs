use serde::{Deserialize, Serialize};
use std::io::{BufReader, BufWriter};
use std::io;
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
    email: String
}

fn csv_to_json(input_path: String, output_path: String) -> io::Result<()> {
    let file = File::open(input_path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(buf_reader);

    let mut records: Vec<Person> = Vec::new();
    for result in reader.deserialize() {
        let record: Person = result?;
        records.push(record);
        println!("{:?}", records);
    }
    // let data = records.iter().collect();
    let json_file = File::create(output_path)?;
    let buf_writer = BufWriter::new(json_file);
    let jotain = serde_json::to_writer_pretty(buf_writer, &records)?;
    println!("jotain: {:?}", jotain);
    Ok(())
}

fn main() {
    let input_path = String::from("example.csv");
    let asd = csv_to_json(input_path, "jsonjsonssss.json".to_string());
    println!("ss{:?}", asd)
}
