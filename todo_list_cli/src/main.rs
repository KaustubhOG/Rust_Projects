use std::io::{self, Write};

fn main() {
    let mut todos: Vec<(String, bool)> = Vec::new(); // (task, is_done)

    loop {
        println!("\n==== TODO CLI ====");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as done/undone");
        println!("4. Delete a task");
        println!("5. Clear all tasks");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter new task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todos.push((task.trim().to_string(), false));
                println!("Task added!");
            }

            "2" => {
                if todos.is_empty() {
                    println!("No tasks yet!");
                } else {
                    println!("\nYour tasks:");
                    for (i, (task, done)) in todos.iter().enumerate() {
                        let status = if *done { "Done" } else { "Pending" };
                        println!("{}. {} [{}]", i + 1, task, status);
                    }
                }
            }

            "3" => {
                if todos.is_empty() {
                    println!("No tasks to mark!");
                    continue;
                }
                print!("Enter task number to toggle done: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();

                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= todos.len() {
                        todos[index - 1].1 = !todos[index - 1].1;
                        println!("Task status toggled!");
                    } else {
                        println!("Invalid task number!");
                    }
                } else {
                    println!("Please enter a valid number!");
                }
            }

            "4" => {
                if todos.is_empty() {
                    println!("No tasks to delete!");
                    continue;
                }
                print!("Enter task number to delete: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();

                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= todos.len() {
                        let removed = todos.remove(index - 1);
                        println!("Removed: {}", removed.0);
                    } else {
                        println!("Invalid task number!");
                    }
                } else {
                    println!("Please enter a valid number!");
                }
            }

            "5" => {
                if todos.is_empty() {
                    println!("No tasks to clear!");
                } else {
                    todos.clear();
                    println!("All tasks cleared!");
                }
            }

            "6" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice! Try again."),
        }
    }
}
