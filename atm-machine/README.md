# ATM Machine Simulation

This project simulates an ATM machine with limited cash, focusing on memory safety and resource management using Rust's ownership and RAII principles.

## Overview

The ATM machine is designed to handle cash withdrawals while ensuring that the cash store is managed safely. The project consists of several modules that work together to provide a seamless user experience while adhering to best practices in memory management.

### Features

- **Cash Management**: The ATM can store and manage a limited amount of cash, allowing users to withdraw funds as long as the balance permits.
- **Memory Safety**: The project implements ownership tracking to prevent memory leaks and double frees, ensuring safe memory usage throughout the application.
- **RAII Principles**: Resources are automatically released when the ATM shuts down, following the RAII (Resource Acquisition Is Initialization) paradigm.
- **Memory Awareness**: Utility functions are included to print memory addresses and segment information (stack/heap) during operations, providing insights into memory usage.

## Project Structure

- `src/main.rs`: Entry point of the application. Initializes the ATM machine and manages user input for withdrawals.
- `src/atm.rs`: Contains the `ATM` struct with methods for handling cash withdrawals and managing the cash store.
- `src/cash.rs`: Defines the `Cash` struct representing the total amount of cash available in the ATM, with methods for adding and withdrawing cash.
- `src/utils.rs`: Utility functions for printing memory addresses and segment information during operations.
- `Cargo.toml`: Configuration file for the Rust project, specifying package details and dependencies.

## Running the Application

To run the ATM machine simulation, follow these steps:

1. Ensure you have Rust installed on your machine.
2. Clone the repository or download the project files.
3. Navigate to the project directory.
4. Run the following command to start the application:

   ```
   cargo run
   ```

5. Follow the on-screen instructions to perform cash withdrawals.

## Conclusion

This ATM machine simulation serves as an educational tool for understanding memory management in Rust, showcasing the importance of ownership, borrowing, and RAII in building safe and efficient applications.