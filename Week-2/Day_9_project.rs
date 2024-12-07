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

