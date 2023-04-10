// use clap::builder::Str;

#[derive(Debug)]

pub(crate) struct Task {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub category: String,
    pub completed: bool,
}

// impl Task {

//     // Constructor
//     pub fn new(title: String, description: String, due_date: String, category: String) -> Self {
//         Task {
//             title,
//             description,
//             due_date,
//             category,
//             completed: false,
//         }
//     }

    // Convert Task to String
    // pub fn to_string(&self) -> String {
    //     let completed_str = if self.completed { "Yes" } else { "No" };
    //     format!(
    //         "Title: {}\nDescription: {}\nDue Date: {}\nCategory: {}\nCompleted: {}",
    //         self.title, self.description, self.due_date, self.category, completed_str
    //     )
    // }
    // Task format
    // fn to_string(&self) -> String {
    //     format!(
    //     "Title: {}
    //         \nDescription: {}\nDue Date: {}
    //         \nCategory: {}\nCompleted: {}",

    //         self.title,
    //         self.description,
    //         self.due_date,
    //         self.category,
    //         self.completed
    // )
    // }


    // Parse string representation and create Task instance
    // pub fn from_string(task_str: &str) -> Result<Self, String> {
    //     let task_parts: Vec<&str> = task_str.split('\n').collect();
    //     if task_parts.len() < 5 {
    //         return Err(String::from("Invalid task string"));
    //     }

    //     let title = task_parts[0].replace("Title: ", "");
    //     let description = task_parts[1].replace("Description: ", "");
    //     let due_date = task_parts[2].replace("Due Date: ", "");
    //     let category = task_parts[3].replace("Category: ", "");
    //     let completed_str = task_parts[4].replace("Completed: ", "");
    //     let completed = completed_str == "Yes";

    //     Ok(Task::new(title, description, due_date, category).set_completed(completed))
    // }

    // Helper method to set completed field
    // pub fn set_completed(mut self, completed: bool) -> Self {
    //     self.completed = completed;
    //     self
    // }
    // create task from string
    // fn from_string(task_string: &str) -> Result<Task, String>{
    //     let fields: Vec<&str> = task_string.lines().collect(); 
    //     if fields.len() != 5 {
    //         return Err("Invalid task string".to_string())
    //     }

    //     Ok(Task {
    //         title: fields[0].to_string(),
    //         description: fields[1].to_string(),
    //         due_date: fields[2].to_string(),
    //         category: fields[3].to_string(),
    //         completed: fields[4].parse().map_err(|_| "Invalid task string")?,
    //     })
    // }
// }

