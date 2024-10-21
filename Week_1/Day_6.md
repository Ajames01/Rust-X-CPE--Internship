# Simple calculator in Rust
   ```

2. Open a terminal and navigate to the project directory.

3. Compile and run the program using Cargo:
   ```bash
   cargo run
   ```

4. Follow the prompts to enter two numbers and an operation.

### Example Usage

```
Enter the first number
5
Enter the second number
3
Enter the operation
+
The result is: 8
```

## Code Explanation

- **Input Handling**: The program uses `std::io` to read user input from the terminal.
- **Data Types**: The numbers are stored as `f32` for floating-point arithmetic.
- **Conditional Statements**: The program uses `if` statements to determine which operation to perform based on user input.
- **Error Handling**: The program checks for division by zero and handles invalid inputs gracefully.
