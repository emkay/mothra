use std::fs::{create_dir};
use std::path::{Path, PathBuf};
use std::error::Error;

use dirs;

pub struct FilesManager {
    pub home_dir: PathBuf,
    pub full_path: PathBuf,
}

impl FilesManager {
    pub fn new() -> Result<FilesManager, Box<dyn Error>> {
        let home_dir_result = match dirs::home_dir() {
            Some(dir) => Ok(Path::new(&dir).join(".mothra")),
            None => Err("Path doesn't exist"),
        };

        let home_dir = home_dir_result?;
        let full_path = home_dir.join("tasks.json");

        Ok(FilesManager {
            home_dir,
            full_path,
        })
    }

    pub fn create_mothra_dir(&self) -> Result<(), Box<dyn Error>> {
        if !self.home_dir.exists() {
            println!("Creating dir for: {}", self.home_dir.display());
            create_dir(&self.home_dir)?;
        }

        Ok(())
    }
}
