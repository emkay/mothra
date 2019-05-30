use std::fs::{File};
use std::io::prelude::*;
use std::error::Error;

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

    match cmd {
        Some(c) => {
            if c == "add" {
                println!("add command");

                match value {
                    Some(v) => {
                        let mut ts = Tasks::new();

                        let display = file_path.display();

                        if file_path.exists() {
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
                        }
                    },
                    None => println!("no value!"),
                }
            }
        },
        None => println!("No command given"),
    }

    Ok(())
}
