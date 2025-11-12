use colored::*;
use std::io::{self, Write};

fn print_banner() {
    println!("\n");
    println!("{}", "╔════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                                                                ║".bright_cyan());
    println!("{}", "║  ████████╗ ██████╗ ██████╗  ██████╗         ██████╗██╗     ██╗ ║".bright_cyan());
    println!("{}", "║  ╚══██╔══╝██╔═══██╗██╔══██╗██╔═══██╗       ██╔════╝██║     ██║ ║".bright_cyan());
    println!("{}", "║     ██║   ██║   ██║██║  ██║██║   ██║       ██║     ██║     ██║ ║".bright_cyan());
    println!("{}", "║     ██║   ██║   ██║██║  ██║██║   ██║       ██║     ██║     ██║ ║".bright_cyan());
    println!("{}", "║     ██║   ╚██████╔╝██████╔╝╚██████╔╝▄█╗    ╚██████╗███████╗██║ ║".bright_cyan());
    println!("{}", "║     ╚═╝    ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝     ╚═════╝╚══════╝╚═╝ ║".bright_cyan());
    println!("{}", "║                                                                ║".bright_cyan());
    println!("{}", "║              Your Simple Task Management Tool                  ║".bright_yellow());
    println!("{}", "║                                                                ║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!("\n");
}

fn main() {
    let mut todos: Vec<(String, bool)> = Vec::new(); // (task, is_done)
    
    // Print banner once at startup
    print_banner();

    loop {
        println!("\n{}", "==== TODO CLI ====".bright_magenta().bold());
        println!("{}. Add task", "1".bright_white());
        println!("{}. View tasks", "2".bright_white());
        println!("{}. Mark task as done/undone", "3".bright_white());
        println!("{}. Delete a task", "4".bright_white());
        println!("{}. Clear all tasks", "5".bright_white());
        println!("{}. Exit", "6".bright_white());
        print!("{}", "Enter your choice: ".bright_yellow());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("{}", "Enter new task: ".bright_yellow());
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                todos.push((task.trim().to_string(), false));
                println!("{}", "✓ Task added!".bright_green().bold());
            }

            "2" => {
                if todos.is_empty() {
                    println!("{}", "No tasks yet!".bright_red());
                } else {
                    println!("\n{}", "Your tasks:".bright_magenta().bold());
                    for (i, (task, done)) in todos.iter().enumerate() {
                        if *done {
                            println!("{}. {} {}", 
                                format!("{}", i + 1).bright_white(),
                                task.green().strikethrough(),
                                "[✓ Done]".bright_green().bold()
                            );
                        } else {
                            println!("{}. {} {}", 
                                format!("{}", i + 1).bright_white(),
                                task.bright_white(),
                                "[○ Pending]".bright_yellow()
                            );
                        }
                    }
                }
            }

            "3" => {
                if todos.is_empty() {
                    println!("{}", "No tasks to mark!".bright_red());
                    continue;
                }
                print!("{}", "Enter task number to toggle done: ".bright_yellow());
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();

                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= todos.len() {
                        todos[index - 1].1 = !todos[index - 1].1;
                        println!("{}", "✓ Task status toggled!".bright_green().bold());
                    } else {
                        println!("{}", "✗ Invalid task number!".bright_red());
                    }
                } else {
                    println!("{}", "✗ Please enter a valid number!".bright_red());
                }
            }

            "4" => {
                if todos.is_empty() {
                    println!("{}", "No tasks to delete!".bright_red());
                    continue;
                }
                print!("{}", "Enter task number to delete: ".bright_yellow());
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();

                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= todos.len() {
                        let removed = todos.remove(index - 1);
                        println!("{} {}", "✓ Removed:".bright_green().bold(), removed.0.bright_white());
                    } else {
                        println!("{}", "✗ Invalid task number!".bright_red());
                    }
                } else {
                    println!("{}", "✗ Please enter a valid number!".bright_red());
                }
            }

            "5" => {
                if todos.is_empty() {
                    println!("{}", "No tasks to clear!".bright_red());
                } else {
                    todos.clear();
                    println!("{}", "✓ All tasks cleared!".bright_green().bold());
                }
            }

            "6" => {
                println!("{}", "Goodbye! 👋".bright_cyan().bold());
                break;
            }

            _ => println!("{}", "✗ Invalid choice! Try again.".bright_red()),
        }
    }
}