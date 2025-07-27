
use crate::logger::Logger;

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger
    }
}

impl Logger for ConsoleLogger {
    fn log(&mut self, message: &str) {
        println!("{}", message);
    }
}