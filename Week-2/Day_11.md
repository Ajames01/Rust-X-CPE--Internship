# Inventory Management System with Pattern Matching in Rust

This Rust-based inventory management system demonstrates the use of **pattern matching** for categorizing items based on their category. It includes functionality to manage, display, and categorize items in a structured and readable format.

---

## Features
- Add items to the inventory.
- Display all items in the inventory.
- Categorize items by their category using Rust's `match` expression.

---

## Code Overview

### **Category Enum**
Represents the different categories an item can belong to:
```rust
#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}
```

### **Item Struct**
Represents individual items in the inventory:
```rust
struct Item {
    id: i32,
    name: String,
    category: Category,
    quantity: i32,
    price: i32,
}
```

### **Inventory Struct**
Holds the collection of items and provides methods for managing and displaying them:
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

#### 4. **`categorize_items`**
Uses **pattern matching** to group items by their category:
```rust
fn categorize_items(&self) {
    println!("\nCategorized Inventory:");
    for item in &self.items {
        match item.category {
            Category::Electronics => println!(
                "Electronics -> ID: {}, Name: {}, Quantity: {}, Price: ${}",
                item.id, item.name, item.quantity, item.price
            ),
            Category::Groceries => println!(
                "Groceries -> ID: {}, Name: {}, Quantity: {}, Price: ${}",
                item.id, item.name, item.quantity, item.price
            ),
            Category::Clothing => println!(
                "Clothing -> ID: {}, Name: {}, Quantity: {}, Price: ${}",
                item.id, item.name, item.quantity, item.price
            ),
        }
    }
}
```

---

## Example Code
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

    // Categorize and display items using pattern matching
    inv.categorize_items();
}
```

---

## Sample Output
When you run the program, the output will be:

```
Inventory:
ID: 1, Name: Phone, Category: Electronics, Quantity: 5, Price: $1000
ID: 2, Name: Shirt, Category: Clothing, Quantity: 3, Price: $20
ID: 3, Name: Bread, Category: Groceries, Quantity: 10, Price: $3

Categorized Inventory:
Electronics -> ID: 1, Name: Phone, Quantity: 5, Price: $1000
Clothing -> ID: 2, Name: Shirt, Quantity: 3, Price: $20
Groceries -> ID: 3, Name: Bread, Quantity: 10, Price: $3
```

---


---

## Key Takeaways
- **Pattern Matching**: The `match` expression is used to group items by their category in a concise and readable way.
- **Scalability**: Adding new categories is straightforward by extending the `Category` enum and `match` arms.
- **Readability**: The categorized inventory is displayed in a clear, organized format.

This program is a practical example of combining control flow and pattern matching in Rust for real-world use cases.
