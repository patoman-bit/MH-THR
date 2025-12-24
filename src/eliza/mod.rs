mod memory;
mod patterns;
mod responses;

use memory::Memory;
use patterns::get_pattern_response;

pub fn start_conversation() {
    let mut memory = Memory::new();
    println!("ELIZA: Hi! I'm here to listen. Type 'Quit' to exit.\n");

    loop {
        use std::io::{self, Write};
        print!("You: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("ELIZA: Goodbye! Take care.\n");
            break;
        }

        memory.add_input(input.to_string());
        let response = get_pattern_response(input, &memory);
        println!("\x1b[32mELIZA: {}\x1b[0m\n", response);
    }
}
