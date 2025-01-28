
**File I/O (Input/Output)** in Rust is performed using the `std::fs` module, 
which provides functions and types for interacting with the file system. Here's a breakdown of the key concepts:

## 1. Opening a File
Rust provides the File type from `std::fs` to open and manipulate files.
- Read a file: Use `File::open` to open a file in read-only mode.
```
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("File content: {}", content);
    Ok(())
}
```

## 2. Writing to a File
To write to a file, use `File::create` (to create or truncate a file) or `OpenOptions` (to append or write conditionally).

```
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;
    Ok(())
}
```
## 3. Using `OpenOptions`
`OpenOptions` allows more control, such as opening a file in read, write, append, or truncate mode.
```
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("example.txt")?;
    file.write_all(b"\nAppending this line!")?;
    Ok(())
}
```
## 4. Handling Errors
File I/O operations can fail (e.g., file not found, permission denied), so you must handle errors using `Result`.
```
use std::fs::File;

fn main() {
    match File::open("nonexistent.txt") {
        Ok(_) => println!("File opened successfully."),
        Err(e) => eprintln!("Error opening file: {}", e),
    }
}

```
## 5. Reading Line by Line
For large files, you can use a `BufReader` to read line by line efficiently.
```
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("example.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```
## 6. Writing Line by Line
Use `BufWriter` for efficient line-by-line writes.
```
use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let file = File::create("example.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "First line")?;
    writeln!(writer, "Second line")?;
    Ok(())
}
```
