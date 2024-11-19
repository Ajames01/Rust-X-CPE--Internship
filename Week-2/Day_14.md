# Error Handling

To make the inventory system more robust, we can add error handling using Rust's built-in features like `Result`, `Option`, and custom error types. This will allow us to handle invalid operations gracefully, such as attempting to retrieve or remove an item that doesn't exist.

---

### Enhancements for Error Handling

1. **Add a Custom Error Type**: Define an enum to represent possible errors in the inventory.
2. **Handle Invalid Item Searches**: Return a `Result` when searching for an item by ID.
3. **Add Item Removal**: Implement a method to remove items with proper error handling.
4. **Use Pattern Matching for Error Propagation**: Leverage `match` to handle `Result` types.

---

### Updated Code with Error Handling

```rust
#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}

#[derive(Debug)]
struct Item {
    id: i32,
    name: String,
    category: Category,
    quantity: i32,
    price: i32,
}

#[derive(Debug)]
struct Inventory {
    items: Vec<Item>,
}

#[derive(Debug)]
enum InventoryError {
    ItemNotFound(i32),
    EmptyInventory,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn display_inventory(&self) {
        if self.items.is_empty() {
            println!("The inventory is empty.");
        } else {
            println!("Inventory:");
            for item in &self.items {
                println!(
                    "ID: {}, Name: {}, Category: {:?}, Quantity: {}, Price: ${}",
                    item.id, item.name, item.category, item.quantity, item.price
                );
            }
        }
    }

    fn find_item_by_id(&self, id: i32) -> Result<&Item, InventoryError> {
        self.items.iter().find(|&item| item.id == id).ok_or(InventoryError::ItemNotFound(id))
    }

    fn remove_item(&mut self, id: i32) -> Result<Item, InventoryError> {
        let index = self.items.iter().position(|item| item.id == id);
        match index {
            Some(idx) => Ok(self.items.remove(idx)),
            None => Err(InventoryError::ItemNotFound(id)),
        }
    }

    fn categorize_items(&self) {
        if self.items.is_empty() {
            println!("\nNo items to categorize.");
        } else {
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
    }
}
```

---

### Example Usage with Error Handling

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

    // Add items to inventory
    inv.add_item(item1);
    inv.add_item(item2);

    // Display inventory
    inv.display_inventory();

    // Find an item by ID
    match inv.find_item_by_id(2) {
        Ok(item) => println!(
            "\nFound item: Name: {}, Category: {:?}, Quantity: {}, Price: ${}",
            item.name, item.category, item.quantity, item.price
        ),
        Err(e) => println!("\nError finding item: {:?}", e),
    }

    // Attempt to remove an item
    match inv.remove_item(3) {
        Ok(item) => println!("\nRemoved item: {:?}", item),
        Err(e) => println!("\nError removing item: {:?}", e),
    }

    // Display categorized items
    inv.categorize_items();
}
```

---

### Key Additions

#### 1. **Custom Error Type**
```rust
#[derive(Debug)]
enum InventoryError {
    ItemNotFound(i32),
    EmptyInventory,
}
```

This enum allows us to handle specific errors like missing items or an empty inventory.

#### 2. **Error Propagation with `Result`**
Methods like `find_item_by_id` and `remove_item` now return `Result` to indicate success or failure.

#### 3. **Safe Item Removal**
The `remove_item` method ensures that attempting to remove a non-existent item results in an appropriate error.

#### 4. **Empty Inventory Handling**
The `display_inventory` and `categorize_items` methods handle the case of an empty inventory gracefully.

---

### Sample Output

**Example 1: Display Inventory**
```
Inventory:
ID: 1, Name: Phone, Category: Electronics, Quantity: 5, Price: $1000
ID: 2, Name: Shirt, Category: Clothing, Quantity: 3, Price: $20
```

**Example 2: Search for Item by ID**
```
Found item: Name: Shirt, Category: Clothing, Quantity: 3, Price: $20
```

**Example 3: Attempt to Remove Non-Existent Item**
```
Error removing item: ItemNotFound(3)
```

**Example 4: Categorized Inventory**
```
Categorized Inventory:
Electronics -> ID: 1, Name: Phone, Quantity: 5, Price: $1000
Clothing -> ID: 2, Name: Shirt, Quantity: 3, Price: $20
```

---

### Benefits of Error Handling
1. **Resilience**: Prevents the program from crashing when operations fail.
2. **Clarity**: Provides meaningful error messages to help debug and understand issues.
3. **Safety**: Avoids unintentional behavior, like accessing invalid indices or modifying missing items.

This updated implementation makes the inventory system robust and production-ready!
