use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Open,
    Closed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    current_id: u32,
    items: HashMap<u32, Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            current_id: 1,
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, description: String, priority: Priority) {
        let task = Task::new(description, priority);
        self.items.insert(self.current_id, task);
        self.current_id += 1;
    }

    pub fn len(self) -> usize {
        self.items.len()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    description: String,
    priority: Priority,
    status: Status,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Task {
    pub fn new(description: String, priority: Priority) -> Task {
        let dt = Utc::now();
        Task { description, priority, status: Status::Open, created: dt, updated: dt }
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
