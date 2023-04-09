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

fn complete_task(task_list: &mut Vec<Task>) {
    println!("Mark a task as completed:")

    // Take user input for the index of the task to mark as completed
    println!("Enter the index of the task to mark as completed:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read index");
    let index = index.trim().parse().expect("Invalid index");

    // Check if the index is within the valid range
    if index < 1 || index > task_list.len() {
        println!("Invalid index. Please enter a valid index.");
        return;
    }

    // Mark the task as completed
    task_list[index - 1].completed = true;

    println!("Task marked as completed successfully!");
}

fn delete_task(task_list: &mut Vec<Task>) {
    println!("Delete a task:");

    // Take user input for the index of the task to delete
    println!("Enter the index of the task to delete:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read index");
    let index = index.trim().parse().expect("Invalid index");

    // Check if the index is within the valid range
    if index < 1 || index > task_list.len() {
        println!("Invalid index. Please enter a valid index.");
        return;
    }

    // Remove the task from the task list
    task_list.remove(index - 1);

    println!("Task deleted successfully!");
}

fn exit_task_manager() {
    println!("Exiting the Task Manager. Goodbye!");
    std::process::exit(0); // Exit the program with a status code of 0
}

fn main() {
    print!("Taskman - A new rust based cli task manager.");

    let mut task_list: Vec<Task> = Vec::new();

    add_task(&mut task_list);

    list_tasks(&mut task_list);

    // Call the complete_task function to mark a task as completed
    complete_task(&mut task_list);


    // Call the delete_task function to delete a task
    delete_task(&mut task_list);

    // Call the exit_task_manager function to exit the task manager
    exit_task_manager();
}