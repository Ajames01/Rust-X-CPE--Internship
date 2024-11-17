
// Import the HashMap package from the standard collections library
use std::collections::HashMap;

// Define a struct to represent a contact
struct Contact {
    name: String,
    phone: String,
    email: String,
}

// Define a struct to represent a contact book
struct ContactBook {
    contacts: HashMap<String, Contact>,
}

// Implement methods for the ContactBook struct
impl ContactBook {
    // Create a new instance of ContactBook
    fn new() -> Self {
        ContactBook {
            contacts: HashMap::new(),
        }
    }

    // Add a contact to the contact book
    fn add_contact(&mut self, contact: Contact) {
        self.contacts.insert(contact.name.clone(), contact);
    // Insert the contact into the contacts HashMap using the contact's name as the key.
    // The name is cloned to avoid moving ownership of the original string.
    }

    // Remove a contact from the contact book by name
    fn remove_contact(&mut self, name: &str) {
        self.contacts.remove(name);
    }

    // Search for a contact by name and return an Option containing a reference to the Contact
    fn search_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }
}

// Main function to demonstrate the functionality of the contact book
fn main() {
    let mut contact_book = ContactBook::new(); // Create a new contact book

    // Add contacts to the contact book
    contact_book.add_contact(Contact {
        name: "James Anaga".to_string(),
        phone: "123-456-7890".to_string(),
        email: "james@example.com".to_string(),
    });

    contact_book.add_contact(Contact {
        name: "Kelly Smith".to_string(),
        phone: "234-567-8901".to_string(),
        email: "kelly@example.com".to_string(),
    });

    // Search for a specific contact in the contact book
    if let Some(contact) = contact_book.search_contact("James Anaga") {
        println!("Name: {}", contact.name);
        println!("Phone: {}", contact.phone);
        println!("Email: {}", contact.email);
    } else {
        println!("Contact not found");
    }

    // Remove a specific contact from the contact book
    contact_book.remove_contact("James Anaga");
}

