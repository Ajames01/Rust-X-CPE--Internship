# Pattern Matching and Control Flow in Rust

This README provides an overview of pattern matching and control flow constructs in Rust, with examples to help you understand and utilize these features effectively.


## Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to match values against patterns and execute code based on the match.

### Match Expressions

`match` expressions are used to compare a value against a series of patterns and execute the code corresponding to the first matching pattern.

```rust
fn main() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),
    }
}
```

### Patterns

Patterns can include literals, variables, wildcards, and more. They can be used in various contexts, such as function parameters and `let` bindings.

```rust
fn print_number(number: i32) {
    match number {
        0 => println!("Zero"),
        n if n < 0 => println!("Negative number: {}", n),
        n => println!("Positive number: {}", n),
    }
}
```

### Destructuring

Destructuring allows you to break apart complex data types, such as tuples and structs, into their individual components.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    match point {
        Point { x, y } => println!("Point at ({}, {})", x, y),
    }
}
```

## Control Flow

Rust provides several control flow constructs, such as `if`, `else`, `loop`, `while`, and `for` to manage the execution of code based on conditions and iterations.

### if and else

The `if` and `else` constructs are used to execute code based on boolean conditions.

```rust
fn main() {
    let temperature = 25;

    if temperature > 30 {
        println!("It's hot!");
    } else if temperature < 15 {
        println!("It's cold!");
    } else {
        println!("It's comfortable.");
    }
}
```

### Loops

Loops repeatedly execute a block of code. Rust has several types of loops: infinite loops with `loop`, conditional loops with `while`, and iterator-based loops with `for`.

### loop

`loop` creates an infinite loop that can be terminated with `break`.

```rust
fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
        println!("Counter: {}", counter);
    }
}
```

### while

`while` loops execute a block of code as long as a condition is true.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("Number: {}", number);
        number -= 1;
    }
    println!("Liftoff!");
}
```

### for

`for` loops iterate over a collection, such as an array or a range.

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    for number in 1..6 {
        println!("Number: {}", number);
    }
}
```
Pattern matching and control flow constructs in Rust provide powerful tools for managing the execution of code based on conditions and patterns. These features help you write more concise and expressive code.

Feel free to explore and experiment with these concepts to deepen your understanding and improve your Rust programming skills!

---

