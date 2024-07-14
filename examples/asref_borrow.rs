use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct MyString(String);

// impl AsRef<str> for MyString {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

impl Borrow<str> for MyString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

fn main() {
    let mut map: HashMap<MyString, i32> = HashMap::new();
    map.insert(MyString("key".to_string()), 42);

    // Look up using a string slice
    let value = map.get("key");
    println!("Value: {:?}", value); // Outputs: Some(42)
}
