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

## Conclusion

Understanding ownership and borrowing in Rust is crucial for writing safe and efficient code. This README provides a foundational overview of these concepts with code examples. Feel free to explore further and experiment with your own Rust programs!

