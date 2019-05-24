pub mod tasks {
    use std::collections::HashMap;

    use chrono::{DateTime, Utc};
    use serde::{Serialize, Deserialize};

    /// The `Priority` type is used for assigning a priority to a `Task`.
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

        pub fn add(&mut self, task: Task) {
            self.items.insert(self.current_id, task);
            self.current_id += 1;
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
}
