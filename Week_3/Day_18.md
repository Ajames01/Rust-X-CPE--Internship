# Exploring Iterators and Closures in Rust

## Introduction
Welcome to my Rust journey! In this repository, you'll find my explorations of iterators and closures in Rust—two powerful features that enhance the language's efficiency and expressiveness.


## Iterators

### Introduction
An iterator is an object that allows you to access the elements of a collection one at a time, without needing to know the underlying implementation details of the collection.

Iterators in Rust are implemented using the `Iterator` trait, which defines methods that allow you to perform various operations on the elements of a collection, such as filtering, mapping, and reducing.

### Example: Summing Numbers in a Vector
Here's an example of how you might use an iterator to sum up the numbers in a vector:

```rust
let numbers = vec![1, 2, 3];
let sum: i32 = numbers.iter().sum();
println!("The sum is: {}", sum); // Output: The sum is: 6
```

In this example, we create a `Vec` of numbers, and then use the `iter()` method to create an iterator over the elements of the vector. We then call the `sum()` method on the iterator to calculate the sum of all the numbers.

### Example: Filtering and Mapping
Iterators can also be used to perform more complex operations, such as filtering and mapping:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<i32> = numbers.iter().filter(|n| *n % 2 == 0).collect();
println!("The even numbers are: {:?}", even_numbers); // Output: The even numbers are: [2, 4]
```

In this example, we use the `filter()` method to create a new iterator that only includes the even numbers from the original vector. We then use the `collect()` method to collect the even numbers into a new `Vec`.

## Closures

### Introduction
Closures in Rust are anonymous functions that can capture variables from their surrounding environment. This allows you to write concise and expressive code that can be passed as arguments to other functions, such as iterators.

### Example: Using a Closure with an Iterator
Here's an example of how you might use a closure with an iterator:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<i32> = numbers.iter().filter(|&n| n % 2 == 0).collect();
println!("The even numbers are: {:?}", even_numbers); // Output: The even numbers are: [2, 4]
```

In this example, we use a closure as the argument to the `filter()` method of the iterator. The closure is `|&n| n % 2 == 0`, which checks whether the current element is even. The closure captures the `n` variable from the surrounding environment and dereferences it using the `&` operator to access the value of the element.

Closures in Rust can also capture variables by reference or by value, depending on the needs of the closure. They are a powerful tool for writing concise and expressive code, and they work particularly well with iterators to perform complex operations on collections of data.

## Full Example: Combining Iterators and Closures
Here's a complete example that demonstrates the use of iterators and closures in Rust:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using iter() to create an iterator
    let sum: i32 = numbers.iter().sum();
    println!("The sum is: {}", sum); // Output: The sum is: 15

    // Using filter() with a closure to filter even numbers
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&n| n % 2 == 0).collect();
    println!("The even numbers are: {:?}", even_numbers); // Output: The even numbers are: [2, 4]

    // Using map() with a closure to square each number
    let squared_numbers: Vec<i32> = numbers.iter().map(|&n| n * n).collect();
    println!("The squared numbers are: {:?}", squared_numbers); // Output: The squared numbers are: [1, 4, 9, 16, 25]

    // Using into_iter() to take ownership and modify the collection
    let doubled_numbers: Vec<i32> = numbers.into_iter().map(|n| n * 2).collect();
    println!("The doubled numbers are: {:?}", doubled_numbers); // Output: The doubled numbers are: [2, 4, 6, 8, 10]
}
```
Iterators and closures are powerful features in Rust that enable you to write clean, efficient, and expressive code. Whether you're filtering, mapping, or reducing collections, these tools make your life easier and your code more readable.

