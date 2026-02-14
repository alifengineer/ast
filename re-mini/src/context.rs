use std::collections::HashMap;

use crate::value::Value;

pub struct DataContext {
    facts: HashMap<String, Value>,
}

impl DataContext {
    pub fn new() -> DataContext {
        Self {
            facts: HashMap::new()
        }
    }

    pub fn add(&mut self, name: String, value: Value){
        self.facts.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<&Value> {
        self.facts.get(&name) // we borrow here
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.facts.insert(name, value);
    }
}