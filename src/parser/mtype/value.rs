enum Value {
    Number  { val: i32 },
    Str     { val: String },
    Bool    { val: bool }
}

impl Value {
    fn new_num(val: i32) -> Value {
        Value::Number { val }
    }

    fn new_str(val: impl Into<String>) -> Value {
        Value::Str { val: val.into() }
    }

    fn new_bool(val: bool) -> Value {
        Value::Bool { val }
    }
}

#[test]
fn test_value_new() {
    let n = Value::new_num(1204);
    let s = Value::new_str("Gochiusa");
    let b = Value::new_bool(true);
    let v_list = vec![n, s, b];

    for value in v_list {
        match value {
            Value::Number { val } => assert_eq!(val, 1204),
            Value::Str { val } => assert_eq!(val, "Gochiusa"),
            Value::Bool { val } => assert!(val)
        }
    }
}
