mod task;

use std::io;
use task::Task;


fn add_task(task_list: &mut Vec<Task>){
    println!("ADd a new task");

    // User task input
    println!("Title:");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read title");

    title = title.trim().to_string();

    // description input
    println!("Description:");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description");

    description = description.trim().to_string();

    // Deu date input
    println!("Due Date:");
    let mut due_date = String::new();
    io::stdin()
        .read_line(&mut due_date)
        .expect("Failed to read due date");

    due_date = due_date.trim().to_string();

    // Category input
    println!("Category:");
    let mut category = String::new();
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read category");

    category = category.trim().to_string();

    // Completed
    println!("Completed (true/false):");
    let mut completed = String::new();
    io::stdin()
        .read_line(&mut completed)
        .expect("Failed to read completed status");

    completed= completed.trim().parse().expect("Invalid completion status.");


    // create a new task
    let task = Task {
        title,
        description,
        due_date,
        category,
        completed,
    };
    task_list.push(task);
    println!("Task added successfully!")
}

fn list_tasks(task_list: &mut Vec<Task>) {
    println!("All Tasks:");

    for (index, task) in task_list.iter().enumerate() {
        println!("Task {}: ", index + 1);
        println!("Title: {}", task.title);
        println!("Description: {}", task.description);
        println!("Due Date: {}", task.due_date);
        println!("Category: {}", task.category);
        println!("Completed: {}", task.completed);
        println!();
    }
}
fn main() {
    print!("Taskman - A new rust based cli task manager.");

    let mut task_list: Vec<Task> = Vec::new();

    add_task(&mut task_list);

    list_tasks(&mut task_list)
}