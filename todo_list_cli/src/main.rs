use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n==== TODO CLI ====");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Exit");
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
                todos.push(task.trim().to_string());
                println!("✅ Task added!");
            }
            "2" => {
                if todos.is_empty() {
                    println!("No tasks yet!");
                } else {
                    println!("\nYour tasks:");
                    for (i, task) in todos.iter().enumerate() {
                        println!("{}: {}", i + 1, task);
                    }
                }
            }
            "3" => {
                println!("Goodbye 👋");
                break;
            }
            _ => println!("Invalid choice! Try again."),
        }
    }
}
