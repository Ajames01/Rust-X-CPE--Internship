# Traits
Traits in Rust are similar to interfaces in other languages. 
They define a set of methods that a type must implement. 
Traits allow you to define shared behavior across different types.

**Example**
```
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("Woof!")
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("Meow!")
    }
}

fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };

    println!("Dog says: {}", dog.make_sound());
    println!("Cat says: {}", cat.make_sound());
}
```
In this example:
- The `Animal` trait defines a method `make_sound`.
- The `Dog` and `Cat` structs implement the Animal trait, providing their own versions of `make_sound`.

# Generics 
Generic allow you to write code that works with different types while maintaining type safety.
They enable you to define functions, structs, enums, and methods that can operate on multiple types without sacrifacing performance.
```
struct Container<T> {
    item : T
}

impl <T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
    fn get_item(&self) -> &T {
        &self.item
    }
}

fn main() {
    let num_container = Container::new(42);
    let string_container = Container::new("Hello".to_string());
    println!("The item in the container is: {}", num_container.get_item());
    println!("The item in the container is: {}", string_container.get_item());
}
```
In this example:
- The `Container` struct is generic over a type `T`
- The `new` method creates a new `Container` instance, and the `get_item` method returns a reference to the contained item.
- The `main` function demonstrates creating `Container` instances for different types (`i32` and `String`).
