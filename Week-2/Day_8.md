# Overview of Structs in Rust

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

