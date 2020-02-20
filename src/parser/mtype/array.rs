use super::value::Value;

pub struct Array {
    values: Vec<Value>
}

impl Array {
    fn new() -> Array {
        Array { values: Vec::new() }
    }

    fn push(self: &mut Array, val: Value) {
        self.values.push(val)
    }

    fn len(self: &Array) -> usize {
        self.values.len()
    }
}

#[test]
fn test_array() {
    let mut array = Array::new();
    array.push(Value::new_num(410));
    array.push(Value::new_bool(true));
    array.push(Value::new_str("Cocoa"));
    assert_eq!(array.len(), 3);
}
