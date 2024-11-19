use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
    email: String,
}

struct ContactBook {
    contacts: HashMap<String, Contact>,
}

impl ContactBook {
    fn new() -> Self {
        ContactBook {
            contacts: HashMap::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) {
        self.contacts.insert(contact.name.clone(), contact);
    }

    fn remove_contact(&mut self, name: &str) {
        self.contacts.remove(name);
    }

    fn search_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }

    fn list_contacts(&self, sort_by: Option<&str>, filter_by: Option<&str>) -> Vec<&Contact> {
        let mut contacts: Vec<&Contact> = self.contacts.values().collect();

        if let Some(field) = sort_by {
            contacts.sort_by(|a, b| match field {
                "name" => a.name.cmp(&b.name),
                "phone" => a.phone.cmp(&b.phone),
                "email" => a.email.cmp(&b.email),
                _ => panic!("Invalid sort field: {}", field),
            });
        }

        if let Some(filter) = filter_by {
            contacts = contacts.into_iter()
                .filter(|c| c.name.contains(filter) || c.phone.contains(filter) || c.email.contains(filter))
                .collect();
        }

        contacts
    }
}

fn main() {
    let mut contact_book = ContactBook::new();

    // Add some contacts
    contact_book.add_contact(Contact {
        name: "John Doe".to_string(),
        phone: "123-456-7890".to_string(),
        email: "john.doe@example.com".to_string(),
    });
    contact_book.add_contact(Contact {
        name: "Jane Smith".to_string(),
        phone: "987-654-3210".to_string(),
        email: "jane.smith@example.com".to_string(),
    });
    contact_book.add_contact(Contact {
        name: "Bob Johnson".to_string(),
        phone: "555-555-5555".to_string(),
        email: "bob.johnson@example.com".to_string(),
    });

    // List contacts sorted by name
    let sorted_contacts = contact_book.list_contacts(Some("name"), None);
    println!("Contacts sorted by name:");
    for contact in sorted_contacts {
        println!("{} - {} ({})", contact.name, contact.phone, contact.email);
    }
    println!();

    // List contacts filtered by "o"
    let filtered_contacts = contact_book.list_contacts(None, Some("o"));
    println!("Contacts containing 'o':");
    for contact in filtered_contacts {
        println!("{} - {} ({})", contact.name, contact.phone, contact.email);
    }
}
