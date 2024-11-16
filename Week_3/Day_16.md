

# HashMap Example in Rust

This repository contains an example of using `HashMap` in Rust. A `HashMap` is a collection that stores data in key-value pairs, allowing for efficient data retrieval and manipulation.

## Code Explanation

The following Rust code demonstrates how to create, manipulate, and interact with a `HashMap`. 

### Code

```rust
use std::collections::HashMap; // Import the HashMap struct

fn main() {
    let mut hashmap = HashMap::new(); // Create a new HashMap 

    // Insert key-value pairs into the HashMap
    hashmap.insert("Arduino", 98);
    hashmap.insert("Matlab", 80);
    hashmap.insert("Python", 95);
    hashmap.insert("C/C++", 90);

    // Find the length of the HashMap
    println!("The length of the HashMap is {}", hashmap.len());

    // Get a single value from the HashMap
    match hashmap.get("C/C++") {
        Some(value) => println!("You got {} for C/C++", value),
        None => println!("You didn't study C/C++"),
    }

    // Remove a value from the HashMap
    hashmap.remove("Matlab");

    // Loop through the HashMap
    for (key, value) in hashmap.iter() {
        println!("For {} you got {}%", key, value);
    }

    // Check for a value in the HashMap
    println!("Did you study CSS? {}", hashmap.contains_key("CSS"));
}
```

### Features

- **Insertion**: Add key-value pairs to the `HashMap`.
- **Retrieval**: Access values using their corresponding keys.
- **Removal**: Delete entries from the `HashMap`.
- **Iteration**: Loop through all key-value pairs.
- **Existence Check**: Verify if a specific key exists in the `HashMap`.

## How to Run

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/hashmap-example.git
   ```
3. Navigate to the project directory:
   ```bash
   cd hashmap-example
   ```
4. Compile and run the code:
   ```bash
   cargo run
   ```

## Conclusion

This example illustrates basic operations with `HashMaps` in Rust. Feel free to modify and expand upon this code to explore more features and functionalities!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```

### Instructions:
- Replace `yourusername` with your actual GitHub username in the clone URL.
- You can add any additional sections or information as needed, such as contributing guidelines or acknowledgments.
