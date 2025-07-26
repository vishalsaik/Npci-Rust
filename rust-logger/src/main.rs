mod logger;
mod file_logger;
mod console_logger;

use file_logger::FileLogger;
use console_logger::ConsoleLogger;
use logger::Logger;

fn main() {
    let mut file_logger = FileLogger::new("log.txt").expect("Failed to create file logger");
    let mut console_logger = ConsoleLogger::new();

    file_logger.log("Logging to file!");
    console_logger.log("Logging to console!");
}