// This file contains utility functions and types for the logger project. 
// It may include error handling and validation functions that can be used across different logger implementations.

use std::fs::File;
use std::io::{self, Write};

pub fn write_to_file(file_path: &str, message: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;
    writeln!(file, "{}", message)?;
    Ok(())
}

pub fn handle_critical_error(error: &str) {
    panic!("Critical error occurred: {}", error);
}