use std::fs::{File};
use std::io::prelude::*;
use std::error::Error;
use std::path::PathBuf;
use std::io::BufReader;

use tabular::{Table, Row};

use mothra::fs::FilesManager;
use mothra::tasks::{Tasks, Priority};

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<dyn Error>> {
    let mut args = args.skip(1);

    let fm_result = FilesManager::new();
    let fm = fm_result?;

    fm.create_mothra_dir()?;

    let file_path = fm.full_path;
    let cmd = args
        .next();

    let value = args
        .next();

    let mut ts: Tasks;

    if file_path.exists() {
        let file_result = read_file(&file_path);
        let content = file_result?;
        ts = serde_json::from_str(&content).unwrap();
    } else {
        ts = Tasks::new();
    }
    match cmd {
        Some(c) => {
            if c == "add" {
                match value {
                    Some(v) => {
                        let display = file_path.display();

                        ts.add(String::from(v), Priority::Low);

                        let serialized = serde_json::to_string(&ts).unwrap();
                        let mut file = match File::create(&file_path) {
                            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
                            Ok(file) => file,
                        };

                        match file.write_all(serialized.as_bytes()) {
                            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
                            Ok(_) => println!("successfully wrote to {}", display),
                        }
                    },
                    None => println!("no value!"),
                }
            }
        },
        None => {
            let mut table = Table::new("{:<}   {:<}   {:<}   {:<}   {:<}");

            let header = Row::new()
                .with_cell("Description")
                .with_cell("Priority")
                .with_cell("Status")
                .with_cell("Created")
                .with_cell("Updated");

            table.add_row(header);

            for task in ts.items.values() {
                let row = Row::new()
                    .with_cell(&task.description)
                    .with_cell(&task.priority)
                    .with_cell(&task.status)
                    .with_cell(&task.created)
                    .with_cell(&task.updated);

                table.add_row(row);
            }

            println!("{}", table);
        }
    }

    Ok(())
}

fn read_file(filepath: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    Ok(contents)
}
