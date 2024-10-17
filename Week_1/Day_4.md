# Functions in Rust



- **Function**: A function is a block of code that performs a specific task. It can take inputs, process them, and return an output. Functions help organize code and promote reusability.

- **Parameters**: Parameters are variables that are passed into a function. They allow you to provide input data to the function when it is called.

- **Return Value**: A return value is the output that a function produces after executing its code. Functions can return values to the caller, which can then be used in further calculations or logic.

#
## Code Examples

### 1. Function Without Parameters

This example demonstrates a simple function that prints a message.

```rust
fn main() {
    print_something();
}

fn print_something() {
    println!("Hello! I'm loving Rust!!");
}
### 1. Function Without Parameters

This example demonstrates a simple function that prints a message.

```rust
fn main() {
    print_something();
}

fn print_something() {
    println!("Hello! I'm loving Rust!!");
}
```

### 2. Function With Parameters

This example shows how to define a function that calculates the volume of a rectangular prism using provided dimensions.

```rust
fn main() {
    let length = 5.0;
    let width = 10.0;
    let height = 20.1;
    calculate_volume(length, width, height);
}

fn calculate_volume(l: f32, b: f32, h: f32) {
    let volume = l * b * h;
    println!("The volume of the shape is {}", volume);
}
```

### 3. Function With Return Value

In this example, the function calculates the volume and returns it, allowing for further use in the program.

```rust
fn main() {
    let length = 5.0;
    let width = 10.0;
    let height = 20.1;  
    let volume = calculate_volume(length, width, height);
    println!("Volume is {}", volume);
}

fn calculate_volume(l: f32, b: f32, h: f32) -> f32 {
    let volume = l * b * h;
    volume
}
```


