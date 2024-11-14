# String Methods, Pattern Matching and Control Flow in Rust

# Understanding String Methods in Rust


## Introduction

In Rust, the `String` type is a mutable, growable string stored in the heap. This document will explore several common methods for manipulating strings, including replacing substrings, splitting strings into lines or tokens, trimming whitespace, and accessing individual characters.

## String Replacement

The `replace` method is used to create a new string by replacing all occurrences of a specified substring with another substring.

```rust
fn main() {
    let my_string = String::from("newbie");
    println!("I am a {} in rust", my_string.replace("newbie", "getting better"));
}
```

### Explanation:
- In this example, the word "newbie" is replaced with "getting better". The `replace` method returns a new string without modifying the original.

## Line Splitting

The `lines` method splits a string into an iterator of its lines, allowing you to process each line individually.

```rust
fn main() {
    let my_string = String::from("The weather is \n nice \n outside! ");
    for line in my_string.lines() {
        println!("{}", line);
    }
}
```

### Explanation:
- The `lines` method splits the string at newline characters (`\n`) and iterates through each line, printing them one by one.

## String Splitting

The `split` method divides a string into substrings based on a specified delimiter.

```rust
fn main() {
    let my_string = String::from("Leave+a+like+if+you+enjoyed+this!");
    let tokens: Vec<&str> = my_string.split("+").collect();
    println!("{}", my_string);
    println!("At index 2: {}", tokens[2]);
}
```

### Explanation:
- Here, the string is split at each occurrence of the `+` character. The resulting substrings are collected into a vector. The example prints the original string and accesses the token at index 2.

## Trimming Whitespace

The `trim` method removes leading and trailing whitespace from a string.

```rust
fn main() {
    let my_string = String::from("  My name is James   \n\r");
    println!("Before trim: {}", my_string);
    println!("After trim: {}", my_string.trim());
}
```

### Explanation:
- This example shows how to remove unnecessary spaces and newline characters from both ends of the string using `trim`.

## Character Access

The `chars` method returns an iterator over the characters of a string, allowing you to access characters by their index.

```rust
fn main() {
    let my_string = String::from("AJames.rs on X");
    match my_string.chars().nth(4) {
        Some(c) => println!("Character at index 4: {}", c),
        None => println!("No character at index 4")
    }
}
```

### Explanation:
- In this case, `chars().nth(4)` retrieves the character at index 4. If the index is out of bounds, it handles the case gracefully by returning `None`.

## Conclusion

This README has covered essential string methods in Rust, providing examples that illustrate how to manipulate strings effectively. Understanding these methods will enhance your ability to work with text data in Rust applications. For further reading, refer to the official Rust documentation on [String](https://doc.rust-lang.org/std/string/struct.String.html).




# Pattern Matching

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



