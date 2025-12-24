use regex::Regex;

use super::memory::Memory;

pub fn get_pattern_response(input: &str, memory: &Memory) -> String {
    let patterns = vec![
        (r"(?i)I need (.*)", "Why do you need {}?"),
        (
            r"(?i)Why can't I (.*)",
            "What makes you think you can't {}?",
        ),
        (r"(?i)I feel (.*)", "Why do you feel {}?"),
        (r"(?i)Hello", "Hello! How can I help you today?"),
    ];

    for (pattern, response) in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(input) {
                return response.replace("{}", caps.get(1).map_or("", |m| m.as_str()));
            }
        }
    }

    if let Some(last_input) = memory.get_last() {
        return format!("Can you elaborate on '{}'", last_input);
    }

    "Tell me more.".to_string()
}
