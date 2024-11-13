```rust
use std::collections::HashMap; // Import the HashMap struct

fn main() {
    let mut hashmap = HashMap::new(); // creates new HashMap 
    // hashmap.insert("key", value);
    hashmap.insert("Arduino", 98);
    hashmap.insert("Matlab", 80);
    hashmap.insert("python", 95);
    hashmap.insert("C/C++", 90);

    // find the length of HashMap
    println!("The length of the HashMap is {}", hashmap.len());

    // Get a single value of a HashMap
    match hashmap.get("C/C++") {
        Some(hashmap) => println!("You got {} for C/C++", hashmap),
        None => println!("You didn't study C/C++"),
    }

    // Remove a value from HashMap
    hashmap.remove("Matlab");

    // Loop through the HashMap
    for (key, value) in hashmap.iter() {
        println!("For {} you got {}%", key, value);
    }

    // Check for value in HashMap
    println!("Did you study CSS? {}", hashmap.contains_key("CSS"));
}
```
