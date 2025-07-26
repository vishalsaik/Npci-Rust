use std::fs::File;
use std::io::{self, Write};
use crate::logger::Logger;

pub struct FileLogger {
    file: File,
}

impl FileLogger {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(FileLogger { file })
    }
}

impl Logger for FileLogger {
    fn log(&mut self, message: &str) {
        if let Err(e) = writeln!(self.file, "{}", message) {
            panic!("Critical file write error: {}", e);
        }
    }
}