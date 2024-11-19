# Add Sorting and Filtering Options
I added sorting and filtering options to the contact book app using iterators in Rust. 
 Here's what's new:

1. Added a `list_contacts()` method to the `ContactBook` struct:
    - This method takes two optional parameters: `sort_by` (a field to sort by) and `filter_by` (a string to filter by).
    - It collects all the `Contact` values from the `contacts` HashMap into a `Vec<&Contact>`.
    - If a `sort_by` field is provided, it sorts the `Vec` based on the specified field (name, phone, or email).
    - If a `filter_by` string is provided, it filters the `Vec` to only include contacts where the name, phone, or email contains the filter string.
    - It returns the sorted and filtered `Vec<&Contact>`.


In the `main()` function:

- We call the `list_contacts()` method with different parameters to demonstrate the sorting and filtering capabilities.
- First, we list all the contacts sorted by name.
- Then, we list all the contacts containing the letter "o" in any field.



With these additions, the contact book app now supports sorting the contacts by name, phone, or email, as well as filtering the contacts by a given string. 
This provides more flexibility and control over how the contacts are displayed and managed.
