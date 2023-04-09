#[derive(Debug)]

struct Task {
    task_num: i32,
    title: String,
    description: String,
    due_date: String,
    category: string,
    completed: bool,
}

impl Task {
    // Task format
    fn to_string(&self) -> String {
        format!(
        "Number: {}\nTitle: {}
            \nDescription: {}\nDue Date: {}
            \nCategory: {}\nCompleted: {}",
            self.task_num,
            self.title,
            self.description,
            self.due_date,
            self.category,
            self.completed
    )
    }

    // create task from string
    fn from_string(task_string: &str) -> Result<Self, String>{
        let fields: Vec<str> = task_string.lines().collect(); 
        if fields.len() != 6 {
            return Err("Invalid task string".to_string())
        }

        Ok(Task {
            task_num: fields[0].to_string(),
            title: fields[1].to_string(),
            description: fields[2].to_string(),
            due_date: fields[3].to_string(),
            category: fields[4].to_string(),
            completed: fields[5].parse().map_err(|_| "Invalid task string")?,
        })
    }
}
