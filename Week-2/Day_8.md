# Overview of Structs & Enums in Rust

Structs in Rust are user-defined data types that allow you to group related data together. They are essential for creating complex data structures and come in three primary forms: **structs with named fields**, **tuple structs**, and **unit structs**.

## Defining a Struct

To define a struct, you use the `struct` keyword followed by the struct's name and its fields. Each field consists of a name and a data type.

**Example: Defining a Basic Struct**

```rust
struct Person {
    name: String,
    age: u8,
    height: u8,
}
```

In this example, we define a `Person` struct with three fields: `name`, `age`, and `height`. 

## Instantiating a Struct

You create an instance of a struct using the struct's name followed by curly braces containing field values.

**Example: Creating an Instance**

```rust
let person1 = Person {
    name: String::from("John Doe"),
    age: 18,
    height: 178,
};
```

## Accessing Struct Fields

You can access the fields of a struct using dot notation.

**Example: Accessing Fields**

```rust
println!("Name: {}", person1.name);
println!("Age: {}", person1.age);
println!("Height: {}", person1.height);
```

## Mutability

By default, structs are immutable. To modify their fields, you must declare the instance as mutable using the `mut` keyword.

**Example: Making a Struct Mutable**

```rust
let mut person2 = Person {
    name: String::from("Jane Doe"),
    age: 25,
    height: 165,
};

person2.age = 26; // This is valid because person2 is mutable.
```

## Tuple Structs and Unit Structs

- **Tuple Structs** are similar to regular structs but do not have named fields. Instead, they use positional indexing.

**Example: Tuple Struct**

```rust
struct Point(i32, i32);

let point = Point(10, 20);
println!("x: {}, y: {}", point.0, point.1);
```

- **Unit Structs** do not contain any fields and are primarily used as markers or for implementing traits.

**Example: Unit Struct**

```rust
struct Marker;

let marker = Marker; // No fields to initialize.
```

## Destructuring Structs

Rust allows destructuring of structs to extract field values into separate variables.

**Example: Destructuring**

```rust
let Person { name, age, height } = person1;
println!("Name: {}, Age: {}, Height: {}", name, age, height);
```

## Conclusion

Structs in Rust provide a powerful way to create complex data types that can hold multiple related values. They support various features like mutability, destructuring, and different forms (named fields, tuple structs, unit structs), making them versatile for various programming needs.

# ENUMS
**Enums**, short for enumerations, are a type in Rust that allow you to define a value that can be one of several distinct variants. Enums are used to represent data that can have multiple possible states, where each state

### Using Enums
You can create a variable of an enum type by assigning it to one of the enum’s variants. Then, you can use a match expression or an if let statement to handle each variant. may have its own unique structure or no additional data at all.

**Basic Syntax**

Here’s how you define an enum in Rust:

          enum Direction {
              North,
              South,
              East,
              West,
          }

### Enums with Data
Enums in Rust can also store additional data associated with each variant, allowing more complex structures.

Example:

          enum Message {
              Quit,
              Move { x: i32, y: i32 },
              Write(String),
              ChangeColor(u8, u8, u8),
          }

Here, Message has four variants:

1. **Quit**: A simple variant with no associated data.
2. **Move { x: i32, y: i32 }**: A variant with two named fields (x and y), representing coordinates.
3. **Write(String)**: A variant with a single associated String value, representing a text message.
4. **ChangeColor(u8 u8, u8)**: A variant with three u8 values, which can be used to represent RGB color values.

### Example with Pattern Matching

You can use match to access the data inside enum variants:

          fn main() {
              let msg = Message::Write(String::from("AJames));
          
              match msg {
                  Message::Quit => println!("Quit message"),
                  Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
                  Message::Write(text) => println!("My name is {}, I ama Rustacean", text),
                  Message::ChangeColor(R, G, B) => println!("Change color to RGB({}, {}, {})", R, G, B),
              }
          }

We use match msg to check which variant of Message is currently stored in msg. The match expression then runs different code based on the variant.

Each arm of the match expression handles one of the four Message variants:

1. **Message::Quit**: Prints "Quit message".
2. **Message::Move { x, y }**: Destructures the Move variant to access its x and y values and prints them.
3. **Message::Write(text)**: Extracts the text value from Write and prints it.
4. **Message::ChangeColor(R, G, B)**: Destructures ChangeColor to access the R, G, and B values and prints them in an RGB format.

### What the Code Does
Since msg is set to `Message::Write(String::from("A.James"));`, Rust goes through the match arms and finds the `Message::Write(text)` pattern. It then:

Matches this pattern and extracts the sting of text.

Prints: My name is AJames, I am a Rustacean
