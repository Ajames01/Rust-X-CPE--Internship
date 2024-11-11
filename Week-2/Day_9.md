 # Inventory Management System
An inventory management system (IMS) is a technology solution that helps businesses track and manage their inventory levels, orders, sales, and deliveries. It ensures efficient handling of stock throughout its lifecycle, from procurement to sale.
In Rust we make a ibnentory management system with the following steps below:

### 1. Defining the ItemType Enum
```rust
enum ItemType {
    Potion,
    Sword,
    Shield,
    Food,
}
```
This section defines an `ItemType` enum that lists the types of items our inventory can hold. Enums are useful for representing a fixed set of related values. In this case, `Potion`, `Sword`, `Shield`, and `Food` are different item types.

### 2. Defining the Item Struct
```rust
struct Item {
    name: String,
    item_type: ItemType,
    quantity: u32,
}
```
The `Item` struct represents an individual item in the inventory. It includes three fields:
- `name`: A `String` representing the item's name.
- `item_type`: An `ItemType` enum indicating the type of the item.
- `quantity`: A `u32` to denote how many of that item are present.

### 3. Defining the Inventory Struct
```rust
struct Inventory {
    items: Vec<Item>,
}
```
The `Inventory` struct holds a vector of `Item` objects. Vectors (`Vec<T>`) are used because they can grow and shrink dynamically, making them ideal for a collection of items.

### 4. Implementing Inventory Methods
The `Inventory` struct has methods to add, remove, and list items in the inventory.
```rust
impl Inventory {
    // Add a new item to the inventory
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    // Remove an item from the inventory by name
    fn remove_item(&mut self, item_name: &str) {
        self.items.retain(|item| item.name != item_name);
    }

    // List all items in the inventory
    fn list_items(&self) {
        for item in &self.items {
            println!("{}: {:?}", item.name, item.item_type);
        }
    }
}
```
- **add_item**: Takes a mutable reference to the inventory (`&mut self`) and an `Item`. It adds the item to the `items` vector.
- **remove_item**: Takes a mutable reference to the inventory and the name of the item to be removed. It uses `retain` to keep only those items whose names do not match the provided `item_name`.
- **list_items**: Takes an immutable reference to the inventory (`&self`) and prints out each item’s name and type.

### 5. Creating and Using the Inventory in main
```rust
fn main() {
    // Create an inventory
    let mut inventory = Inventory { items: Vec::new() };

    // Add some items to the inventory
    inventory.add_item(Item {
        name: String::from("Health Potion"),
        item_type: ItemType::Potion,
        quantity: 5,
    });

    inventory.add_item(Item {
        name: String::from("Iron Sword"),
        item_type: ItemType::Sword,
        quantity: 1,
    });

    // List the items in the inventory
    inventory.list_items();

    // Remove an item from the inventory
    inventory.remove_item("Health Potion");

    // List the items in the inventory again
    inventory.list_items();
}
```
- We create an empty `Inventory` object with an empty vector.
- We then add two items (`Health Potion` and `Iron Sword`) to the inventory using the `add_item` method.
- We list all items in the inventory.
- We remove the `Health Potion` from the inventory.
- Finally, we list the items again to show the updated inventory after removal.

This simple workflow demonstrates how to manage an inventory system using enums and structs in Rust. If you have any further questions or need additional functionality, feel free to ask!
