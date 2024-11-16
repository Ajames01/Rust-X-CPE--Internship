# Inventory Management System in Rust

This is a simple inventory management system written in Rust. It allows you to manage items in an inventory, search for items by ID or category, and display the entire inventory in a structured format.

---

## Features
- Add items to the inventory.
- Display all items in the inventory.
- Search for an item by its unique ID.
- Filter items by their category.

---

## Code Overview

### `Category` Enum
Represents the categories an item can belong to:
```rust
#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}
```

### `Item` Struct
Represents an individual item in the inventory:
```rust
struct Item {
    id: i32,
    name: String,
    category: Category,
    quantity: i32,
    price: i32,
}
```

### `Inventory` Struct
Holds all items and provides methods for managing the inventory:
```rust
struct Inventory {
    items: Vec<Item>,
}
```

### Methods in `Inventory`
#### 1. **`new`**
Creates a new, empty inventory:
```rust
fn new() -> Inventory {
    Inventory { items: Vec::new() }
}
```

#### 2. **`add_item`**
Adds an item to the inventory:
```rust
fn add_item(&mut self, item: Item) {
    self.items.push(item);
}
```

#### 3. **`display_inventory`**
Displays all items in the inventory:
```rust
fn display_inventory(&self) {
    println!("Inventory:");
    for item in &self.items {
        println!(
            "ID: {}, Name: {}, Category: {:?}, Quantity: {}, Price: ${}",
            item.id, item.name, item.category, item.quantity, item.price
        );
    }
}
```

#### 4. **`find_item_by_id`**
Finds an item by its ID:
```rust
fn find_item_by_id(&self, id: i32) -> Option<&Item> {
    self.items.iter().find(|&item| item.id == id)
}
```

#### 5. **`find_items_by_category`**
Finds all items belonging to a specific category:
```rust
fn find_items_by_category(&self, category: Category) -> Vec<&Item> {
    self.items
        .iter()
        .filter(|&item| item.category == category)
        .collect()
}
```

---

## Example Usage
```rust
fn main() {
    let mut inv = Inventory::new();

    let item1 = Item {
        id: 1,
        name: "Phone".to_string(),
        category: Category::Electronics,
        quantity: 5,
        price: 1000,
    };

    let item2 = Item {
        id: 2,
        name: "Shirt".to_string(),
        category: Category::Clothing,
        quantity: 3,
        price: 20,
    };

    let item3 = Item {
        id: 3,
        name: "Bread".to_string(),
        category: Category::Groceries,
        quantity: 10,
        price: 3,
    };

    // Add items to inventory
    inv.add_item(item1);
    inv.add_item(item2);
    inv.add_item(item3);

    // Display all items in the inventory
    inv.display_inventory();

    // Find an item by ID
    if let Some(item) = inv.find_item_by_id(2) {
        println!(
            "\nFound item with ID 2: Name: {}, Category: {:?}, Quantity: {}, Price: ${}",
            item.name, item.category, item.quantity, item.price
        );
    } else {
        println!("\nItem with ID 2 not found.");
    }

    // Find items by category
    let electronics = inv.find_items_by_category(Category::Electronics);
    println!("\nElectronics in inventory:");
    for item in electronics {
        println!(
            "ID: {}, Name: {}, Quantity: {}, Price: ${}",
            item.id, item.name, item.quantity, item.price
        );
    }
}
```

---

## Sample Output
```
Inventory:
ID: 1, Name: Phone, Category: Electronics, Quantity: 5, Price: $1000
ID: 2, Name: Shirt, Category: Clothing, Quantity: 3, Price: $20
ID: 3, Name: Bread, Category: Groceries, Quantity: 10, Price: $3

Found item with ID 2: Name: Shirt, Category: Clothing, Quantity: 3, Price: $20

Electronics in inventory:
ID: 1, Name: Phone, Quantity: 5, Price: $1000
```





## Possible Extensions
- Add functionality to update item quantities.
- Implement item removal from the inventory.
- Provide sorting options (e.g., by price or name).
- Persist inventory data using file storage.

---
