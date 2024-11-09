# Rust Ownership and Borrowing

This repository contains examples and explanations of ownership and borrowing in Rust, concepts that are fundamental to memory safety and concurrency in the language. Understanding these concepts is essential for writing efficient and safe Rust programs.

## Table of Contents

- [Ownership](#ownership)
- [Borrowing](#borrowing)
  - [Immutable Borrowing](#immutable-borrowing)
  - [Mutable Borrowing](#mutable-borrowing)
- [Conclusion](#conclusion)

## Ownership

In Rust, ownership is a set of rules that governs how memory is managed. Each value in Rust has a single owner, and when the owner goes out of scope, the value is dropped (freed). Here are some key points:

1. **Transferring Ownership**: When you assign a value to another variable, ownership is transferred. The original variable can no longer be used.

   ```rust
   fn main() {
       let s1 = String::from("Hello World");
       let s2 = s1; // Transferring ownership
       println!("{}", s2); // This works
       // println!("{}", s1); // This would cause an error
   }
   ```

2. **Cloning Values**: To make both variables coexist, you can clone the value. This creates a deep copy of the data.

   ```rust
   fn main() {
       let s1 = String::from("Hello World");
       let s2 = s1.clone(); // Cloning the value
       println!("S1= {}, S2 = {}", s1, s2);
   }
   ```

   **Output**:
   ```
   S1= Hello World, S2 = Hello World
   ```

## Borrowing

Borrowing allows you to reference a value without taking ownership. There are two types of borrowing: immutable and mutable.

### Immutable Borrowing

With immutable borrowing, you can read the value but cannot modify it. You can create multiple immutable references to the same data.

```rust
fn main() {
    let s1 = String::from("Hello World");
    let s2 = &s1; // Immutable borrowing

    println!("{}", s2); // This works
}
```

You can have multiple immutable references:

```rust
fn main() {
    let s1 = String::from("Hello World");
    let s2 = &s1; // Immutable borrowing
    let s3 = &s1; // Another immutable borrow

    println!("{} {}", s2, s3); // This works
}
```

### Mutable Borrowing

Mutable borrowing allows you to modify the borrowed value. However, you can only have one mutable reference at a time.

```rust
fn main() {
    let mut s1 = String::from("Hello World");
    let s4 = &mut s1; // Mutable borrowing
    println!("{}", s4); // This works
}
```

You can modify the borrowed variable:

```rust
fn main() {
    let mut s1 = String::from("Hello World");
    let s4 = &mut s1; // Mutable borrowing
    s4.push_str(" A.James"); // Modifying the borrowed variable 
    println!("{}", s4); // Output: Hello World A.James
}
```

### Ownership Rules Recap

- You can have multiple immutable references (`&T`).
- You can have only one mutable reference (`&mut T`) at a time.
- You cannot mix mutable and immutable references.

```rust
fn main() {
    let mut s1 = String::from("Hello World");

    let s2 = &s1; // Immutable borrowing
    // let s4 = &mut s1; // This would cause an error if uncommented
    
    println!("{}", s2);
}
```
## Error Handling in Rust

Error handling is a critical aspect of Rust programming, emphasizing safety and reliability. Rust uses a combination of `Result` and `Option` types to manage errors effectively without relying on exceptions. This approach allows developers to handle errors explicitly, making the code more robust and maintainable.

### Key Concepts

1. **Result Type**:
   - The `Result` type is an enum that can be either `Ok(T)` for successful outcomes or `Err(E)` for errors. This allows functions to return error information alongside successful results.
   - Example:
     ```rust
     fn divide(a: f64, b: f64) -> Result<f64, String> {
         if b == 0.0 {
             Err("Cannot divide by zero".to_string())
         } else {
             Ok(a / b)
         }
     }
     ```

2. **Option Type**:
   - The `Option` type indicates the presence (`Some(T)`) or absence (`None`) of a value, useful for scenarios where a value may not exist.
   - Example:
     ```rust
     fn find_item(id: i32) -> Option<Item> {
         // Search logic here
     }
     ```

3. **Error Propagation**:
   - Rust provides the `?` operator to propagate errors easily. If a function returns a `Result`, using `?` will return the error immediately if it occurs.
   - Example:
     ```rust
     fn process() -> Result<(), String> {
         let value = divide(10.0, 0.0)?;
         println!("Result: {}", value);
         Ok(())
     }
     ```

4. **Custom Error Types**:
   - Developers can create custom error types using enums to categorize different error scenarios, improving clarity and handling.
   - Example:
     ```rust
     #[derive(Debug)]
     enum MyError {
         Io(std::io::Error),
         Parse(std::num::ParseIntError),
         Other(String),
     }
     ```

### Implementing Error Handling in a Calculator

To illustrate error handling in Rust, we can modify a simple calculator program to include robust error management.

Here’s an enhanced version of a basic calculator that handles user input errors effectively:

```rust
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        },
    };

    input.clear();
    println!("Enter the second number: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid number.");
            return;
        },
    };

    println!("Select operation:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid operation selection! Please enter a number between 1 and 4.");
            return;
        },
    };

    let result = match operation {
        1 => x + y,
        2 => x - y,
        3 => x * y,
        4 => {
            if y == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            x / y
        },
        _ => {
            println!("Invalid operation! Please select a valid option.");
            return;
        },
    };

    println!("The result is: {}", result);
}
```

### Explanation of the Code

- **Input Handling**: The program reads user inputs for two numbers and an operation type, using `match` statements to handle potential parsing errors.
- **Operation Selection**: It checks for valid operations and handles division by zero specifically, returning appropriate error messages.
- **User Feedback**: Clear messages are provided for invalid inputs, ensuring users understand what went wrong.

This structured approach to error handling not only improves user experience but also enhances code reliability by preventing unexpected crashes due to invalid inputs.
