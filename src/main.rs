mod eliza;
mod map;
mod tasks;

const COMMANDS: &[(&str, &str)] = &[
    ("ELIZA", "Start the ELIZA conversational assistant"),
    ("MAP", "Open the map visualization and routing tool"),
    ("TASKS", "Use the lightweight task menu"),
    ("HELP", "Show this menu again"),
    ("QUIT", "Exit the MU/TH/UR terminal"),
];

fn main() {
    println!("\nWelcome to MU/TH/UR AI Terminal System.");
    print_menu();

    loop {
        use std::io::{self, Write};
        print!("MU/TH/UR: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim();

        match input.to_lowercase().as_str() {
            "eliza" => {
                println!("\nInitializing ELIZA...");
                eliza::start_conversation();
            }
            "map" => {
                println!("\nOpening Map Visualization...");
                map::start_map_module();
            }
            "tasks" => {
                println!("\nAccessing Task Automation...");
                tasks::task_interface();
            }
            "help" => {
                print_menu();
            }
            "quit" => {
                println!("\nShutting down MU/TH/UR. Goodbye!");
                break;
            }
            "" => println!("Type a command or 'HELP' to see options."),
            _ => {
                println!("MU/TH/UR: Command not recognized.");
                print_menu();
            }
        }
    }
}

fn print_menu() {
    println!("Available Commands:");
    for (cmd, desc) in COMMANDS {
        println!("  {:<6} - {}", cmd, desc);
    }
    println!();
}
