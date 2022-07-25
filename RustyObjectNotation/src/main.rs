use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyStruct {
    boolean: bool,
    float: f32,
}

fn main() {
    let x: MyStruct = ron::from_str("(boolean: true, float: 1.23)").unwrap();
    
    println!("RON: {}", ron::to_string(&x).unwrap());
}

