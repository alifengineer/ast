
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64)
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            _ => Err(format!("cannot add {:?} and {:?}", self, other)),
        }
    }
}