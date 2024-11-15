# Rust Error Handling: `Result` and `Option`

## Introduction

Rust provides powerful and flexible ways to handle errors through the `Result` and `Option` enums. These enums allow developers to write robust code that gracefully handles unexpected situations and avoids common pitfalls like null pointer dereferencing or unhandled exceptions.

## The `Option` Enum

The `Option` enum is used to represent values that can be either `Some` (containing a value) or `None` (no value).

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### Usage Example

```rust
fn find_user_name(user_id: u32) -> Option<String> {
    if user_id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn main() {
    let user_name = find_user_name(1);
    match user_name {
        Some(name) => println!("User name is {}", name),
        None => println!("User not found"),
    }
}
```

In this example, `find_user_name` returns an `Option<String>`, where `Some` contains the user's name, and `None` indicates the user was not found.

## The `Result` Enum

The `Result` enum is used for functions that can return an error. It has two variants: `Ok` (indicating success) and `Err` (indicating failure).

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Usage Example

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(4.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

In this example, `divide` returns a `Result<f64, String>`, where `Ok` contains the result of the division, and `Err` contains an error message if the divisor is zero.

## Error Handling Techniques

### Unwrapping

`unwrap` and `expect` can be used to get the value from `Option` or `Result`, but they will panic if the value is `None` or `Err`.

```rust
let user_name = find_user_name(1).unwrap();
let result = divide(4.0, 2.0).expect("Division failed");
```

### Matching

Using `match` expressions allows for more controlled handling of `Option` and `Result` values.

```rust
let user_name = find_user_name(2);
match user_name {
    Some(name) => println!("User name is {}", name),
    None => println!("User not found"),
}

let result = divide(4.0, 0.0);
match result {
    Ok(val) => println!("Result: {}", val),
    Err(e) => println!("Error: {}", e),
}
```

### Combinators

Combinators like `map`, `and_then`, `unwrap_or`, and `unwrap_or_else` provide functional ways to work with `Option` and `Result`.

#### `map`
The `map` combinator applies a function to the value inside an `Option` or `Result`, if it exists, and returns a new `Option` or `Result`.

```rust
let some_number = Some(5);
let incremented = some_number.map(|x| x + 1);
println!("{:?}", incremented); // Output: Some(6)

let ok_value: Result<i32, &str> = Ok(10);
let incremented = ok_value.map(|x| x + 1);
println!("{:?}", incremented); // Output: Ok(11)
```

#### `and_then`
The `and_then` combinator, also known as `flatmap` in other languages, applies a function that returns an `Option` or `Result`, and then flattens the result.

```rust
let some_number = Some(5);
let result = some_number.and_then(|x| if x % 2 == 0 { Some(x / 2) } else { None });
println!("{:?}", result); // Output: None

let ok_value: Result<i32, &str> = Ok(10);
let result = ok_value.and_then(|x| if x % 2 == 0 { Ok(x / 2) } else { Err("Not even") });
println!("{:?}", result); // Output: Ok(5)
```

#### `unwrap_or`
The `unwrap_or` combinator returns the contained value or a default value if the `Option` is `None` or the `Result` is `Err`.

```rust
let some_number = None;
let value = some_number.unwrap_or(10);
println!("{}", value); // Output: 10

let err_value: Result<i32, &str> = Err("Oops");
let value = err_value.unwrap_or(10);
println!("{}", value); // Output: 10
```

#### `unwrap_or_else`
The `unwrap_or_else` combinator is similar to `unwrap_or`, but it takes a closure that produces a default value.

```rust
let some_number = None;
let value = some_number.unwrap_or_else(|| 10);
println!("{}", value); // Output: 10

let err_value: Result<i32, &str> = Err("Oops");
let value = err_value.unwrap_or_else(|_| 10);
println!("{}", value); // Output: 10
```

#### `or`
The `or` combinator returns the first `Some` or `Ok` value it encounters, or the second one if the first is `None` or `Err`.

```rust
let first = None;
let second = Some(2);
let result = first.or(second);
println!("{:?}", result); // Output: Some(2)

let first: Result<i32, &str> = Err("First error");
let second: Result<i32, &str> = Ok(2);
let result = first.or(second);
println!("{:?}", result); // Output: Ok(2)
```

#### `or_else`
The `or_else` combinator is similar to `or`, but it takes a closure that produces an alternative `Option` or `Result`.

```rust
let first = None;
let result = first.or_else(|| Some(2));
println!("{:?}", result); // Output: Some(2)

let first: Result<i32, &str> = Err("First error");
let result = first.or_else(|_| Ok(2));
println!("{:?}", result); // Output: Ok(2)
```


Rust's `Option` and `Result` enums provide robust and expressive error handling mechanisms. They encourage developers to explicitly handle potential errors, leading to safer and more reliable code.
