use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self};

use chrono::{DateTime, Utc};
use dirs;
use serde::{Serialize, Deserialize};

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<dyn Error>> {

    let result = match dirs::home_dir() {
        Some(dir) => {
            Ok(Path::new(&dir).join(".mothra"))
        },
        None => Err("Path doesn't exist"),
    };

    let path = result?;

    if !path.exists() {
        println!("Creating dir for: {}", &path.display());
        create_dir(&path)?;
    }

    let mut args = args.skip(1);

    let cmd = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the command argument is missing"))?;

    let sub_cmd = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the sub command argument is missing"))?;

    let value = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "there is no value"))?;

    println!("cmd: {:?}", cmd);
    println!("sub_cmd: {:?}", sub_cmd);
    println!("value: {:?}", value);

    let mut tasks = Tasks::new();

    tasks.add(Task::new(String::from("this is a cool task."), Priority::Low));
    tasks.add(Task::new(String::from("hello another bad boy"), Priority::Medium));
    tasks.add(Task::new(String::from("just some rad tasks"), Priority::High));

    let serialized = serde_json::to_string(&tasks).unwrap();
    let file_path = path.join("tasks.json");
    let display = file_path.display();

    let mut file = match File::create(&file_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    current_id: u32,
    items: HashMap<u32, Task>,
}

impl Tasks {
    fn new() -> Tasks {
        Tasks {
            current_id: 1,
            items: HashMap::new(),
        }
    }

    fn add(&mut self, task: Task) {
        self.items.insert(self.current_id, task);
        self.current_id += 1;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    priority: Priority,
    status: Status,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Task {
    fn new(description: String, priority: Priority) -> Task {
        let dt = Utc::now();
        Task { description, priority, status: Status::Open, created: dt, updated: dt }
    }
}
