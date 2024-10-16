
## Table of Contents
- [Data Types](#data-types)
  - [Integers](#integers)
  - [Floating Point Numbers](#floating-point-numbers)
  - [Boolean](#boolean)
  - [Characters](#characters)
  - [Tuples](#tuples)
  - [Arrays](#arrays)
  - [Vectors](#vectors)


Rust is a systems programming language that emphasizes safety and performance. Understanding its data types is crucial for effective programming.

## Data Types

### Integers
Integers are whole numbers. In Rust, you can specify the size of the integer using types like `i32` (32-bit signed integer).

```rust
let age: i32 = 16; // Correctly assigning an integer value.
```

### Floating Point Numbers
Floating point numbers are used for decimal values. You can use `f32` or `f64` for single or double precision.

```rust
let price: f32 = 19.99; // Example of a floating-point number.
```

### Boolean
Booleans represent truth values: `true` or `false`. They are often used in conditional statements.

```rust
let passed_exam: bool = true;

if passed_exam {
    println!("Congratulations! You passed the exam.");
} else {
    println!("Don't worry, keep studying and try again!");
}
```

### Characters
Characters represent single letters or emojis. In Rust, they are defined using the `char` type.

```rust
let grade: char = 'A';
println!("Your Internship grade is {}", grade);
```

### Tuples
Tuples allow you to group different types of values together, functioning like a mini-package that can hold multiple items.

```rust
let student_info: (String, char) = (String::from("James"), 'A');
```

### Arrays
Arrays are fixed-size lists of elements of the same type. You define an array by specifying its type and size.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
```

### Vectors
Vectors are dynamic arrays that can grow or shrink in size. They are defined using the `Vec` type.

```rust
let mut numbers: Vec<i32> = vec![1, 2, 3]; // Correctly creating a mutable vector.
```
