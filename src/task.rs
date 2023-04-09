#[derive(Debug)]

pub(crate) struct Task {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub category: String,
    pub completed: String,
}

impl Task {
    // Task format
    fn to_string(&self) -> String {
        format!(
        "Title: {}
            \nDescription: {}\nDue Date: {}
            \nCategory: {}\nCompleted: {}",

            self.title,
            self.description,
            self.due_date,
            self.category,
            self.completed
    )
    }

    // create task from string
    fn from_string(task_string: &str) -> Result<Task, String>{
        let fields: Vec<&str> = task_string.lines().collect(); 
        if fields.len() != 5 {
            return Err("Invalid task string".to_string())
        }

        Ok(Task {
            title: fields[0].to_string(),
            description: fields[1].to_string(),
            due_date: fields[2].to_string(),
            category: fields[3].to_string(),
            completed: fields[4].parse().map_err(|_| "Invalid task string")?,
        })
    }
}

