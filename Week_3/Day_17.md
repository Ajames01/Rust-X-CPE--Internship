# Contact Book App

The Contact Book App is a simple application built in Rust that allows users to manage a collection of contacts.

## Data Structures

### `Contact`
The `Contact` struct represents a single contact and contains the following fields:
- `name`: The name of the contact (a `String`)
- `phone`: The phone number of the contact (a `String`)
- `email`: The email address of the contact (a `String`)

### `ContactBook`
The `ContactBook` struct is the main data structure that manages the collection of contacts. It has the following field:
- `contacts`: A `HashMap` that stores the contacts, using the contact's name as the key and the `Contact` struct as the value.

## Functions

### `new()`
The `new()` function is the constructor for the `ContactBook` struct. It creates a new, empty `ContactBook` instance.

### `add_contact()`
The `add_contact()` function takes a `Contact` struct as an argument and adds it to the `contacts` HashMap in the `ContactBook` struct. If a contact with the same name already exists, it will be overwritten.

### `remove_contact()`
The `remove_contact()` function takes the name of a contact (a `&str`) as an argument and removes the corresponding `Contact` struct from the `contacts` HashMap in the `ContactBook` struct.

### `search_contact()`
The `search_contact()` function takes the name of a contact (a `&str`) as an argument and returns an optional reference to the corresponding `Contact` struct in the `contacts` HashMap. If the contact is not found, it returns `None`.

## Usage Example

The `main()` function provides an example of how to use the `ContactBook` app:
1. Create a new `ContactBook` instance.
2. Add two contacts to the `ContactBook`.
3. Search for a contact by name and print the result.
4. Remove a contact by name.
