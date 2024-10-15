# Hello, World!
Now that I’ve installed Rust, it’s time to write my first Rust program. 
It’s traditional when learning a new language to write a little program that prints the text Hello, world! to the screen, so we’ll do the same here!

# Creating a Project Directory
I’ll start by making a directory to store my Rust code. Open a terminal and enter the following commands to make a projects directory 

cargo new project
cd project

This opens the new project in visual studio code

>code .


# Variables in Rust

	Variables are memory locations for storing data
We define variables in rust using **let** followed by the variable name eg. age

To restructure your Rust code for inclusion in a GitHub repository, it's important to organize it clearly, ensuring that it follows best practices for readability and maintainability. Below is the revised code, along with explanations and comments that can be included in a `README.md` file to help others understand the structure and purpose of the code.

### Directory Structure
```
rust_variables/
├── src/
│   └── main.rs
└── README.md
```

### Code in `src/main.rs`
```rust
fn main() {
    // Immutable variable
    let age = 10; // Age is immutable by default
    println!("Age = {}", age);

    // Mutable variable
    let mut mutable_age = 10; // Declare a mutable variable
    mutable_age = 30; // Change the value of mutable_age
    println!("After changing, mutable_age = {}", mutable_age);

    // Scalar data types
    let id: i64 = 73; // Long integer
    let pi: f32 = 3.14; // Floating-point number
    let is_shown: bool = true; // Boolean value
    let first: char = 'a'; // Character

    // Display scalar data types
    println!("ID = {}, PI = {}, is_shown = {}, first = {}", id, pi, is_shown, first);
}
```

### Explanation of Code Structure

1. **Immutable Variables**:
   - The variable `age` is defined as immutable. In Rust, variables are immutable by default, meaning their values cannot be changed after they are set.
   - The output will show the initial value of `age`.

2. **Mutable Variables**:
   - The variable `mutable_age` is declared with the `mut` keyword, allowing its value to be changed later in the program.
   - After changing its value from 10 to 30, the updated value is printed.

