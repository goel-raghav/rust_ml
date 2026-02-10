use std::ops::{Add, Mul};

#[derive(Debug)]
struct Value {
    value: f64,
    children: Vec<Value>,
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other:Value) -> Value {
        Value {
            value: self.value + other.value,
        }
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other:Value) -> Value {
        Value {
            value: self.value * other.value,
        }
    }
    
}

fn main() {
    let a = Value{value: 10.0};
    let b = Value{value: 20.0};
    let value = a + b;
    println!("Value: {:?}", value);
}
