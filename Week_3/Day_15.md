# Vector (`Vec<T>`)
In Rust, a vector `(Vec<T>)` is a growable, heap-allocated array that can store multiple values of the same type, `T`.
## Key Aspects of Vectors in Rust
1. **Declaration and Initialization:**

- You can create an empty vector and add elements to it later, or you can initialize a vector with values.
- Example of an empty vector
```
  let mut vec: Vec<i32> = Vec::new(); // Creates an empty vector of i32
```
- Example with initial values:
```
let vec = vec![1, 2, 3, 4, 5]; // Creates a vector with initial values
```
2. **Adding Elements:**

- You can add elements to a mutable vector using the `push` method.
```
let mut vec = Vec::new();
vec.push(10);
vec.push(20);
vec.push(30);
```
**3. Accessing Elements:**

- Elements in a vector can be accessed using indexing (e.g., `vec[0]`) or the `get` method.
- Indexing (`vec[index]`) returns a reference to the element but can panic if the index is out of bounds.
- `get` method (`vec.get(index)`) returns an `Option<&T>`, which is safer as it handles out-of-bounds cases.
```
let vec = vec![1, 2, 3];
println!("First element: {}", vec[0]);
match vec.get(3) {
    Some(value) => println!("Found: {}", value),
    None => println!("Index out of bounds"),
}
```
4. **Removing Elements:**

- You can remove elements from a vector using methods like `pop`, which removes the last element, or `remove`, which removes an element at a specified index.
```
let mut vec = vec![1, 2, 3];
vec.pop(); // Removes the last element, 3
vec.remove(0); // Removes the element at index 0, which is 1
```
5. **Common Methods:**

- **len():** Returns the number of elements in the vector.
- **is_empty():** Checks if the vector is empty.
- **clear():** Removes all elements from the vector.
- **extend():** Adds elements from an iterator or another collection.
