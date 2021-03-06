use std::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };

        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Open,
    Closed
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Status::Open => "Open",
            Status::Closed => "Closed",
        };

        write!(f, "{}", value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    current_id: u32,
    pub items: HashMap<u32, Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            current_id: 1,
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, description: String, priority: Priority) {
        let task = Task::new(self.current_id, description, priority);
        self.items.insert(self.current_id, task);
        self.current_id += 1;
    }

    pub fn close(&mut self, id: u32) {
        self.items.remove(&id);
    }

    pub fn len(self) -> usize {
        self.items.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: Priority,
    pub status: Status,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, description: String, priority: Priority) -> Task {
        let dt = Utc::now();
        Task { id, description, priority, status: Status::Open, created: dt, updated: dt }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tasks, Priority};

    #[test]
    fn basic() {
        let tasks = Tasks::new();
        assert_eq!(1, tasks.current_id);
        assert_eq!(0, tasks.len());
    }

    #[test]
    fn add_task() {
        let mut tasks = Tasks::new();

        tasks.add(String::from("this is a rad task."), Priority::Low);
        let len = tasks.len();
        assert_eq!(1, len);
    }
}
