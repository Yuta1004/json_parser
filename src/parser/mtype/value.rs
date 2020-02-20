pub enum Value {
    Number  { val: i32 },
    Bool    { val: bool },
    Str     { val: String }
}

impl Value {
    pub fn new_num(val: i32) -> Value {
        Value::Number { val }
    }

    pub fn new_bool(val: bool) -> Value {
        Value::Bool { val }
    }

    pub fn new_str(val: impl Into<String>) -> Value {
        Value::Str { val: val.into() }
    }
}

#[test]
fn test_value_new() {
    let n = Value::new_num(1204);
    let b = Value::new_bool(true);
    let s = Value::new_str("Gochiusa");
    let v_list = vec![n, s, b];

    for value in v_list {
        match value {
            Value::Number { val } => assert_eq!(val, 1204),
            Value::Bool { val } => assert!(val),
            Value::Str { val} => assert_eq!(val, "Gochiusa")
        }
    }
}
