# Project 021 – CRUD Operations on a Text File

## What I Built
A CLI-based Rust app that performs Create, Read, Update, and Delete operations on a plain text file. 

## What I Learned
- I learned how to use std::fs and BufReader to manage file content line-by-line—great for simulating basic text-based databases.
- While Debug trait (derived with #[derive(Debug)]) is good for developers during debugging, Display is meant for end-users. It provides cleaner, more readable messages.

In Code:
```
impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrudError::Io(e) => write!(f, "I/O error: {}", e),
            CrudError::InvalidLineNumber => write!(f, "Invalid line number"),
            CrudError::FileNotFound => write!(f, "File not found"),
        }
    }
}
```

This implementation means:

- When an I/O error occurs, it will show "I/O error: " followed by the specific OS error

- For invalid line numbers, a simple "Invalid line number" message

- For missing files, "File not found"

When this error handling is used in main():
```
match db.read() {
    Ok(lines) => { /* show content */ },
    Err(CrudError::FileNotFound) => println!("📭 File not found"),
    Err(e) => return Err(e),
}
```

## Notes
