use std::collections::HashMap;

use crate::value::Value;

pub struct DataContext {
    facts: HashMap<String, Value>,
}

impl DataContext {
    fn add(&mut self, name: String, value: Value){
        self.facts.insert(name, value);
    }

    fn get(&self, name: String) -> Option<&Value> {
        self.facts.get(&name) // we borrow here
    }

    fn set(&mut self, name: String, value: Value) {
        self.facts.insert(name, value);
    }
}