

```markdown
# Rust Programming: Conditional Statements and Loops

Rust programming.

## Table of Contents
- [Conditional Statements](#conditional-statements)
  - [If Statement](#if-statement)
  - [Else If Statement](#else-if-statement)
  - [Else Statement](#else-statement)
- [Loops](#loops)
  - [Loop](#loop)
  - [While Loop](#while-loop)
  - [For Loop](#for-loop)

## Conditional Statements

Conditional statements in Rust allow for decision-making based on specific conditions. The primary constructs are `if`, `else if`, and `else`. Unlike many other languages, Rust does not require parentheses around conditions.

### If Statement
The `if` statement executes a block of code if the condition evaluates to true.

```rust
fn main() {
    let temp = 25; // Weather temperature
    if temp > 30 {
        println!("The temperature outside is too hot");
    } else if temp < 10 {
        println!("The temperature outside is too cold");
    } else {
        println!("The temperature outside is nice");
    }
}
```

### Else If Statement
The `else if` statement allows for multiple conditions to be evaluated sequentially.

```rust
fn main() {
    let number = 25;
    if number > 40 {
        println!("The number is greater than 40.");
    } else if number > 20 {
        println!("The number is greater than 20.");
    } else {
        println!("The number is 20 or less.");
    }
}
```

### Else Statement
The `else` statement executes when all prior conditions are false.

```rust
fn main() {
    let number = 15;
    if number > 40 {
        println!("The number is greater than 40.");
    } else if number > 20 {
        println!("The number is greater than 20.");
    } else {
        println!("The number is 20 or less.");
    }
}
```

## Loops

Rust provides several types of loops for executing code repeatedly until a condition is met.

### Loop
A basic loop that continues indefinitely until explicitly broken.

```rust
fn main() {
    let mut no_of_interns = 0;
    loop {
        no_of_interns += 1; // Increment by 1
        println!("The No of Interns is {}", no_of_interns);
        if no_of_interns == 5 {
            println!("There are no more slots for interns");
            break; // Exit the loop if the condition is met
        }
    }
}
```

### While Loop
A `while` loop runs as long as the specified condition remains true.

```rust
fn main() {
    let mut no_of_interns = 0;
    while no_of_interns < 5 {
        no_of_interns += 1; // Increment by 1
        println!("The No of Interns is {}", no_of_interns);
        if no_of_interns == 5 {
            println!("There are no more slots for interns");
        }
    }
}
```

### For Loop
A `for` loop iterates over a range of values.

```rust
fn main() {
    for x in 0..5 { // Note: This includes values from 0 to 4, excluding 5.
        println!("The value of i is: {}", x);
    }
}
```
