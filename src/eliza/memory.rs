#[derive(Default)]
pub struct Memory {
    inputs: Vec<String>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { inputs: Vec::new() }
    }

    pub fn add_input(&mut self, input: String) {
        self.inputs.push(input);
    }

    pub fn get_last(&self) -> Option<&String> {
        self.inputs.last()
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn remembers_last_input() {
        let mut memory = Memory::new();
        assert!(memory.get_last().is_none());
        memory.add_input("hello".into());
        memory.add_input("world".into());
        assert_eq!(memory.get_last(), Some(&"world".to_string()));
    }
}
