use std::io::{self, Write}; // importing the io module and the Write trait
// by using the self you can access items in the io module 
// writes - Its a trait that defines methods for bytes to a destination,
// in this case it writes to the standard output

#[derive(Debug)]
enum Category {
    Electronics,
    Groceries,
    Clothing,
}

#[derive(Debug)]
struct Item {
    name: String,
    category: Category,
    quantity: u32,
}

struct Inventory {
    items: Vec<Item>, 
}

impl Inventory {
    // Add a new item to the inventory
    fn add_item(&mut self, item: Item) {
        self.items.push(item); // push the new item into the item vector
    }

    // List all items in the inventory
    fn list_items(&self) {
        for item in &self.items {
            println!("{:?}: {:?}", item.name, item);
        }
    }
}

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
                // Get item details from user
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

fn get_item_from_user() -> Item {
    let mut name = String::new();
    println!("Enter item name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let mut category = String::new();
    println!("Enter item category (1: Electronics, 2: Groceries, 3: Clothing):");
    io::stdin().read_line(&mut category).expect("Failed to read input");

    let item_category = match category.trim() {
        "1" => Category::Electronics,
        "2" => Category::Groceries,
        "3" => Category::Clothing,
        _ => {
            println!("Invalid category. Defaulting to Electronics.");
            Category::Electronics
        }
    };

    let mut quantity = String::new();
    println!("Enter item quantity:");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");

    let item_quantity: u32 = quantity.trim().parse().expect("Please enter a valid number");

    Item {
        name: name.trim().to_string(),
        category: item_category,
        quantity: item_quantity,
    }
}

