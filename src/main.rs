use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::io::{self};

use mothra::tasks::{Task, Tasks, Priority};

use dirs;

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

    let mut ts = Tasks::new();

    ts.add(Task::new(String::from("this is a cool task."), Priority::Low));
    ts.add(Task::new(String::from("hello another bad boy"), Priority::Medium));
    ts.add(Task::new(String::from("just some rad tasks"), Priority::High));

    let serialized = serde_json::to_string(&ts).unwrap();
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
