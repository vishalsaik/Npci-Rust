# Rust Logger

## Overview
This project implements a simple logging system in Rust that can log messages to either a file or the console. It demonstrates the use of traits for generalized logging, file handling, and panic handling in real-world applications.

## Features
- **File Logging**: Logs messages to a specified file.
- **Console Logging**: Outputs log messages to the console.
- **Panic Handling**: On critical failures (e.g., disk full, permissions issue), the logger will panic to ensure that the application is aware of the failure.

## Project Structure
```
rust-logger
├── src
│   ├── main.rs          # Entry point of the application
│   ├── logger.rs        # Trait defining the logging interface
│   ├── file_logger.rs   # Implementation of file logging
│   ├── console_logger.rs # Implementation of console logging
│   └── utils.rs         # Utility functions and types
├── Cargo.toml           # Project configuration file
└── README.md            # Project documentation
```

## Setup Instructions
1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-logger
   ```
3. Build the project:
   ```
   cargo build
   ```

## Usage
To run the logger, execute the following command:
```
cargo run
```
You can modify the `main.rs` file to change the logging behavior (file or console).

## Panic Handling
This project demonstrates two types of panic handling:
- **Unwind**: The default behavior where the stack is unwound, allowing for cleanup of resources.
- **Abort**: A behavior that stops the program immediately without unwinding the stack. This can be enabled in the `Cargo.toml` file by setting the `panic` option to `abort`.

### Example of Panic Handling
In the `file_logger.rs`, if a critical failure occurs (like a full disk), the logger will panic:
```rust
if let Err(e) = write_to_file(message) {
    panic!("Failed to write to log file: {}", e);
}
```

## Conclusion
This Rust Logger project serves as a practical example of file handling, custom traits, and panic management in Rust. It can be extended with additional features such as different log levels or asynchronous logging.