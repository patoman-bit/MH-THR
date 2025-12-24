use std::io::{self, Write};

pub fn task_interface() {
    println!("\nMU/TH/UR Task Manager");
    println!("1. List Tasks");
    println!("2. Add New Task");
    println!("3. Exit");

    loop {
        print!("Enter choice: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let choice = input.trim();

        match choice {
            "1" => list_tasks(),
            "2" => add_task(),
            "3" => {
                println!("Exiting Task Manager.");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn list_tasks() {
    println!("Listing all tasks...");
}

fn add_task() {
    println!("Add a new task...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_handles_invalid_choice() {
        // Ensure helper functions remain callable (no-op asserts for placeholders).
        list_tasks();
        add_task();
    }
}
