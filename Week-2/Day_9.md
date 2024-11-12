 # Inventory Management System
An inventory management system (IMS) is a technology solution that helps businesses track and manage their inventory levels, orders, sales, and deliveries. It ensures efficient handling of stock throughout its lifecycle, from procurement to sale.

The Inventory Management System is a command-line application built with Rust that allows users to manage an inventory of items. Users can add new items, list existing items, and categorize them into predefined categories such as Electronics, Groceries, and Clothing. This application serves as a practical example of using Rust for basic data management tasks.

# Example of an inventory management system in rust

## Features of this Inventory system

- **Add Items**: Users can input details for new items, including name, category, and quantity.
- **List Items**: Users can view all items currently stored in the inventory.
- **Categorization**: Items can be categorized into three types: Electronics, Groceries, and Clothing.



## Code Explanation

The following sections break down the key components of the code and the steps taken to implement this inventory management system.

### 1. Importing Necessary Modules

```rust
use std::io::{self, Write}; 
```
We import the `io` module from the standard library to handle input and output operations. The `Write` trait allows us to write data to standard output.

### 2. Defining Categories with an Enum

```rust
#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}
```
We define an enum `Category` that represents the different categories of items in our inventory. The `Debug` trait allows us to print instances of this enum for debugging purposes.

### 3. Creating an Item Struct

```rust
#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    quantity: u32,
}
```
The `Item` struct represents an individual item with three fields: `name`, `category`, and `quantity`. We derive `Debug` here as well for easy printing.

### 4. Defining the Inventory Struct

```rust
struct Inventory {
    items: Vec<Item>, 
}
```
The `Inventory` struct contains a vector of `Item`s, allowing us to store multiple items in our inventory.

### 5. Implementing Methods for Inventory Management

```rust
impl Inventory {
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn list_items(&self) {
        for item in &self.items {
            println!("{:?}: {:?}", item.name, item);
        }
    }
}
```
We implement methods for adding items to the inventory and listing all items currently stored. The `add_item` method pushes a new item onto the vector, while `list_items` iterates over each item and prints its name and details.

### 6. Main Function Logic

```rust
fn main() {
    let mut inventory = Inventory { items: Vec::new() };

    loop {
        println!("Choose an option:");
        println!("1. Add item");
        println!("2. List items");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let item = get_item_from_user();
                inventory.add_item(item);
            }
            "2" => {
                inventory.list_items();
            }
            "3" => {
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
```
In the `main` function, we initialize an empty inventory and enter a loop that presents users with options to add items, list items, or exit the application. User input is handled using standard input functions.

### 7. Collecting User Input for New Items

```rust
fn get_item_from_user() -> Item {
    // Code for collecting item details from user...
}
```
This function prompts users for item details (name, category, quantity) and constructs an `Item` instance based on their input.

## Example Interaction

When you run the application, it will display a menu like this:

```
Choose an option:
1. Add item
2. List items
3. Exit
```

Users can select an option by entering its corresponding number and follow the prompts to manage their inventory effectively.

## Contributing

If you would like to contribute to this project or suggest improvements, feel free to submit a pull request or open an issue for discussion.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```
